// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package main

import (
	"fmt"
	"io"
	"log"
	"net/http"

	"github.com/rs/xid"

	"cloud.google.com/go/logging"
	"github.com/gorilla/mux"
	"github.com/swrge/disgo"
)

func (a *App) Handler(w http.ResponseWriter, r *http.Request) {
	a.log.Log(logging.Entry{
		Severity: logging.Info,
		HTTPRequest: &logging.HTTPRequest{
			Request: r,
		},
		Labels:  map[string]string{"arbitraryField": "custom entry"},
		Payload: "Structured logging example.",
	})
	fmt.Fprintf(w, "Hello World!\n")
}

func generateHandler(bot *disgo.Client, config RouteConfig) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Check if the Authorization header matches the bot's token
		authHeader := r.Header.Get("Authorization")
		if authHeader[:4] == "Bot " {
			authHeader = authHeader[4:]
		}
		if authHeader != bot.Authentication.Token {
			log.Printf("Unauthorized request: Authorization header doesn't match bot token: %q", authHeader)
			log.Printf("expected: %q", bot.Authentication.Token)
			http.Error(w, "Invalid Token", http.StatusUnauthorized)
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
		err = disgo.SendRequestByte(
			bot,
			xid,
			routeID,
			resourceID,
			r.Method,
			discordURL,
			contentType,
			reqBody,
			&respBody, // Pass a pointer to allow SendRequest to modify the slice
		)

		if err != nil {
			log.Printf("Discord API request failed: %v", err)
			http.Error(w, fmt.Sprintf("Discord API request failed: %v", err), http.StatusBadGateway)
			return
		}

		// Return the response to the microservice
		w.Header().Set("Content-Type", string(contentType))
		w.WriteHeader(http.StatusOK)
		if _, err := w.Write(respBody); err != nil {
			log.Printf("Failed to write response: %v", err)
		}
	}
}
