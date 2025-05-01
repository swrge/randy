package features

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/swrge/bot-worker/http"
	"github.com/switchupcb/disgo"
)

// HandleSlash routes slash commands to their respective handlers.
func HandleSlash(interaction disgo.Interaction, w http.ResponseWriter) {
	var command = interaction.ApplicationCommand()
	switch command.Name {
	case "ping":
		handlePing(interaction, w)
	case "weather":
		HandleWeatherCommand(interaction, w)
	default:
		// Acknowledge the interaction but indicate the command is not handled here yet.
		// Sending an immediate response is crucial for Discord interactions.
		response := &disgo.CreateInteractionResponse{
			InteractionID:    interaction.ID,
			InteractionToken: interaction.Token,
			InteractionResponse: &disgo.InteractionResponse{
				Type: disgo.FlagInteractionCallbackTypeCHANNEL_MESSAGE_WITH_SOURCE,
				Data: disgo.Messages{
					Content: disgo.Pointer("Unhandled command: " + command.Name),
					// Make the response ephemeral so only the user sees it
					Flags: disgo.Pointer(disgo.FlagMessageEPHEMERAL),
				},
			},
		}
		responseJSON, err := json.Marshal(response)
		if err != nil {
			log.Printf("Error marshaling unhandled command response: %v", err)
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write(responseJSON)
		log.Printf("Unhandled command: %s", command.Name)
	}
}

func handlePing(i disgo.Interaction, w http.ResponseWriter) {
	response := &disgo.CreateInteractionResponse{
		InteractionID:    i.ID,
		InteractionToken: i.Token,
		InteractionResponse: &disgo.InteractionResponse{
			Type: disgo.FlagInteractionCallbackTypeCHANNEL_MESSAGE_WITH_SOURCE,
			Data: &disgo.Messages{
				Content: disgo.Pointer("Pong!"),
			},
		},
	}
	response.Send()

	responseJSON, err := json.Marshal(response)
	if err != nil {
		log.Printf("Error marshaling response: %v", err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}

	// Send immediate response to Discord
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	w.Write(responseJSON)

	// Now handle the actual API request asynchronously
	go forwardToRequester(i)
}

func forwardToRequester(interaction disgo.Interaction) {
	// Create a new HTTP client for bot-requester
	client := http.NewClient()
	
	// Determine what API request to make based on interaction data
	// For example, for a "ping" command, let's send a message to the channel
	channelID := interaction.ChannelID

	// Prepare message payload
	messagePayload := map[string]interface{}{
		"content": "This message was sent via bot-requester!",
	}

	// Send message using the HTTP client
	response, err := client.SendMessage(channelID, messagePayload)
	if err != nil {
		log.Printf("Error sending message via bot-requester: %v", err)
		return
	}

	// Log response
	log.Printf("Response from bot-requester: %s", string(response))
}
