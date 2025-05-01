package features

import (
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"strings"
	"time"

	"github.com/swrge/bot-worker/http"
	"github.com/switchupcb/disgo"
)

// Weather conditions for our simulation
var weatherConditions = []string{
	"Sunny", "Partly Cloudy", "Cloudy", 
	"Rainy", "Thunderstorms", "Snowy", 
	"Foggy", "Windy", "Clear",
}

// Weather emojis to make the response more visually appealing
var weatherEmojis = map[string]string{
	"Sunny": "☀️",
	"Partly Cloudy": "⛅",
	"Cloudy": "☁️",
	"Rainy": "🌧️",
	"Thunderstorms": "⛈️",
	"Snowy": "❄️",
	"Foggy": "🌫️",
	"Windy": "💨",
	"Clear": "🌈",
}

// WeatherData represents weather information
type WeatherData struct {
	City        string
	Temperature int
	Condition   string
	Humidity    int
	WindSpeed   int
}

// Initialize the random number generator
func init() {
	rand.Seed(time.Now().UnixNano())
}

// HandleWeatherCommand processes the /weather slash command
func HandleWeatherCommand(interaction disgo.Interaction, w http.ResponseWriter) {
	// Extract the city from command options
	var city string
	if interaction.Data != nil && interaction.Data.Options != nil {
		for _, option := range interaction.Data.Options {
			if option.Name == "city" && option.Value != nil {
				if cityStr, ok := option.Value.(string); ok {
					city = cityStr
				}
			}
		}
	}

	// Default to a random city if none provided
	if city == "" {
		defaultCities := []string{"New York", "London", "Tokyo", "Sydney", "Paris"}
		city = defaultCities[rand.Intn(len(defaultCities))]
	}

	// First, send an acknowledgment response
	response := &disgo.CreateInteractionResponse{
		InteractionID:    interaction.ID,
		InteractionToken: interaction.Token,
		InteractionResponse: &disgo.InteractionResponse{
			Type: disgo.FlagInteractionCallbackTypeCHANNEL_MESSAGE_WITH_SOURCE,
			Data: &disgo.Messages{
				Content: disgo.Pointer(fmt.Sprintf("🔍 Looking up weather for **%s**...", city)),
			},
		},
	}

	responseJSON, err := json.Marshal(response)
	if err != nil {
		log.Printf("Error marshaling response: %v", err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(responseJSON)

	// Simulate fetching weather data
	go func() {
		// Simulate API latency
		time.Sleep(1 * time.Second)

		// Get simulated weather data
		weatherData := getSimulatedWeather(city)

		// Format a nice message
		message := formatWeatherMessage(weatherData)

		// Create HTTP client for bot-requester
		client := http.NewClient()

		// Prepare follow-up message
		followUpMessage := map[string]interface{}{
			"content": message,
		}

		// Send the follow-up message through bot-requester
		_, err := client.SendMessage(interaction.ChannelID, followUpMessage)
		if err != nil {
			log.Printf("Error sending weather follow-up message: %v", err)
		}
	}()
}

// getSimulatedWeather returns simulated weather data for the given city
func getSimulatedWeather(city string) WeatherData {
	// Generate random weather data
	condition := weatherConditions[rand.Intn(len(weatherConditions))]
	
	// Temperature range based on condition
	var tempMin, tempMax int
	switch condition {
	case "Snowy":
		tempMin, tempMax = -10, 5
	case "Rainy", "Foggy", "Thunderstorms":
		tempMin, tempMax = 5, 15
	case "Cloudy", "Windy":
		tempMin, tempMax = 10, 20
	case "Partly Cloudy":
		tempMin, tempMax = 15, 25
	case "Sunny", "Clear":
		tempMin, tempMax = 20, 35
	default:
		tempMin, tempMax = 15, 30
	}

	return WeatherData{
		City:        city,
		Temperature: rand.Intn(tempMax-tempMin) + tempMin,
		Condition:   condition,
		Humidity:    rand.Intn(60) + 30, // 30-90%
		WindSpeed:   rand.Intn(20) + 5,   // 5-25 km/h
	}
}

// formatWeatherMessage creates a nicely formatted weather message
func formatWeatherMessage(data WeatherData) string {
	emoji := weatherEmojis[data.Condition]
	if emoji == "" {
		emoji = "🌡️"
	}

	// Build message with formatting
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("## Weather for %s %s\n\n", data.City, emoji))
	sb.WriteString(fmt.Sprintf("**Condition:** %s %s\n", data.Condition, emoji))
	sb.WriteString(fmt.Sprintf("**Temperature:** %d°C\n", data.Temperature))
	sb.WriteString(fmt.Sprintf("**Humidity:** %d%%\n", data.Humidity))
	sb.WriteString(fmt.Sprintf("**Wind Speed:** %d km/h\n\n", data.WindSpeed))
	
	// Add a dynamic weather tip
	tips := []string{
		"Don't forget your umbrella! ☔",
		"Perfect day for outdoor activities! 🏄‍♂️",
		"Stay hydrated! 💧",
		"Dress warmly! 🧣",
		"Drive safely in these conditions! 🚗",
		"UV index is high, wear sunscreen! 🧴",
	}
	
	var tipIndex int
	if data.Condition == "Rainy" || data.Condition == "Thunderstorms" {
		tipIndex = 0
	} else if data.Condition == "Sunny" || data.Condition == "Clear" {
		tipIndex = 1
	} else if data.Temperature > 25 {
		tipIndex = 2
	} else if data.Temperature < 10 {
		tipIndex = 3
	} else if data.Condition == "Foggy" || data.Condition == "Windy" {
		tipIndex = 4
	} else if data.Condition == "Sunny" && data.Temperature > 20 {
		tipIndex = 5
	} else {
		tipIndex = rand.Intn(len(tips))
	}
	
	sb.WriteString(fmt.Sprintf("**Tip:** %s", tips[tipIndex]))
	
	return sb.String()
}