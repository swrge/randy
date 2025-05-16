package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/gorilla/mux"
	"github.com/joho/godotenv"
	"github.com/rs/xid"
	"github.com/switchupcb/disgo"
)

func createHandlerFunc(bot *disgo.Client, config RouteConfig) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Check if the Authorization header matches the bot's token
		if authHeader := r.Header.Get("Authorization"); authHeader != bot.Authentication.Token {
			log.Printf("Unauthorized request: Authorization header doesn't match bot token: %s", authHeader)
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		vars := mux.Vars(r)

		// Capture query parameters
		queryParams := ""
		if r.URL.RawQuery != "" {
			queryParams = "?" + r.URL.RawQuery
		}

		// Build the Discord API URL (todo: use string builders)
		discordURL := DiscordBaseURL + config.URLBuilder(vars) + queryParams

		// Generate a unique request ID
		xid := xid.New().String()

		// Get the appropriate route ID and resource ID
		routeFunc, exists := disgo.RateLimitHashFuncs[disgo.RouteIDs[config.RouteID]]
		if !exists {
			panic(fmt.Sprintf("Unsupported route ID: %s", config.RouteID))
		}

		// Get the first parameter from vars (often channel_id or guild_id)
		var resourceID string
		for _, v := range vars {
			resourceID = v
			break
		}

		routeID, resourceID := routeFunc(resourceID)

		// Read request body
		var reqBody []byte
		var err error
		if r.Body != nil {
			reqBody, err = io.ReadAll(r.Body)
			if err != nil {
				log.Printf("Error reading request body: %v", err)
				http.Error(w, "Failed to read request body", http.StatusInternalServerError)
				return
			}
		}

		// Copy request headers, especially Content-Type
		contentType := []byte(r.Header.Get("Content-Type"))
		if contentType == nil {
			contentType = []byte("application/json")
		}

		// Send request to Discord API
		var respBody []byte
		err = disgo.SendRequest(
			bot,
			xid,
			routeID,
			resourceID,
			r.Method,
			discordURL,
			contentType,
			reqBody,
			respBody,
		)

		if err != nil {
			log.Printf("Discord API request failed: %v", err)
			http.Error(w, fmt.Sprintf("Discord API request failed: %v", err), http.StatusBadGateway)
			return
		}

		// Return the response to the client
		w.Header().Set("Content-Type", string(contentType))
		w.WriteHeader(http.StatusOK)
		if _, err := w.Write(respBody); err != nil {
			log.Printf("Failed to write response: %v", err)
		}
	}
}

func main() {
	// Load .env file
	if err := godotenv.Load(); err != nil {
		log.Fatal("Warning: No .env file found or error loading it")
	}

	var TOKEN string
	if TOKEN := os.Getenv("BOT_TOKEN"); TOKEN == "" {
		log.Fatal("BOT_TOKEN environment variable is required")
	} else {
		log.Printf("Using BOT_TOKEN specified in .env")
	}

	// Set default port if not specified
	proxyPort := os.Getenv("PROXY_PORT")
	if proxyPort != "" {
		log.Printf("Using PROXY_PORT specified in .env: %s", proxyPort)
	} else {
		proxyPort = "8088" // Default port
		log.Printf("No PROXY_PORT specified in .env, using default port %s", proxyPort)
	}

	// Initialize the Disgo client
	bot := &disgo.Client{
		Authentication: disgo.BotToken(TOKEN),
		Config: &disgo.Config{
			Gateway: disgo.Gateway{}, // we dont use the gateway
			Request: disgo.DefaultRequest(),
		},
	}

	// Set up the router (note only supports v10)
	router := mux.NewRouter()
	for _, config := range routeConfigs {
		routePath := BaseURL + config.PathPattern
		router.HandleFunc(routePath, createHandlerFunc(bot, config)).Methods(config.Method)
		log.Printf("Registered route: %s %s", config.Method, routePath)
	}
	// Start the server
	log.Printf("Reverse proxy starting on %s", proxyPort)
	if err := http.ListenAndServe(fmt.Sprintf(":%s", proxyPort), router); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}
