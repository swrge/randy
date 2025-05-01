package function

import (
	"bytes"
	"crypto/ed25519"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/switchupcb/disgo"
)

func init() {
	functions.HTTP("DiscordInteraction", discordInteraction)
}

func discordInteraction(w http.ResponseWriter, r *http.Request) {
	publicKey := "XXXXXXXXX"
	signature := r.Header.Get("X-Signature-Ed25519")
	timestamp := r.Header.Get("X-Signature-Timestamp")

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read request body", http.StatusBadRequest)
		return
	}

	sigBytes, err := hex.DecodeString(signature)
	if err != nil {
		http.Error(w, "Invalid signature", http.StatusUnauthorized)
		return
	}

	pubKeyBytes, err := hex.DecodeString(publicKey)
	if err != nil {
		http.Error(w, "Invalid public key", http.StatusInternalServerError)
		return
	}

	message := append([]byte(timestamp), body...)
	if !ed25519.Verify(pubKeyBytes, message, sigBytes) {
		http.Error(w, "Signature verification failed", http.StatusUnauthorized)
		return
	}
	// Signature verified, proceed to process the interaction
	var interaction disgo.Interaction
	if err := json.Unmarshal(body, &interaction); err != nil {
		http.Error(w, "Failed to parse interaction", http.StatusBadRequest)
		return
	}

	// Handle different interaction types
	switch interaction.Type {
	case disgo.FlagApplicationCommandTypeCHAT_INPUT:
		handleSlash(interaction, w)
	default:
		http.Error(w, "Unhandled interaction type", http.StatusBadRequest)
	}
}

func handleSlash(interaction disgo.Interaction, w http.ResponseWriter) {
	var command = interaction.ApplicationCommand()
	switch command.Name {
	case "ping":
		handlePing(interaction, w)
	default:
		http.Error(w, "Unhandled command", http.StatusBadRequest)
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
	// Get bot-requester URL from environment
	requesterURL := os.Getenv("BOT_REQUESTER_URL")
	if requesterURL == "" {
		requesterURL = "http://localhost:8088" // Default fallback
	}

	// Determine what API request to make based on interaction data
	// For example, for a "ping" command, let's send a message to the channel
	channelID := interaction.ChannelID

	// Prepare message payload
	messagePayload := map[string]interface{}{
		"content": "This message was sent via bot-requester!",
	}

	payloadBytes, err := json.Marshal(messagePayload)
	if err != nil {
		log.Printf("Error marshaling message payload: %v", err)
		return
	}

	// Create request to bot-requester
	endpoint := fmt.Sprintf("%s/api/v10/channels/%s/messages", requesterURL, channelID)
	req, err := http.NewRequest("POST", endpoint, bytes.NewBuffer(payloadBytes))
	if err != nil {
		log.Printf("Error creating request: %v", err)
		return
	}

	req.Header.Set("Content-Type", "application/json")

	// Send request
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Printf("Error sending request to bot-requester: %v", err)
		return
	}
	defer resp.Body.Close()

	// Log response
	body, _ := io.ReadAll(resp.Body)
	log.Printf("Response from bot-requester: %s", string(body))
}
