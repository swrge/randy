package function

import (
	"crypto/ed25519"
	"encoding/hex"
	"encoding/json"
	"io"
	"log"
	"net/http"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/switchupcb/disgo"
	"github.com/swrge/bot-worker/features"
	_ "github.com/swrge/bot-worker/features" // Import the features package
)

func init() {
	functions.HTTP("DiscordInteraction", discordInteraction)
}

// verifyDiscordSignature checks the validity of the incoming Discord interaction signature.
// It writes an HTTP error and returns false if verification fails.
// It returns true if the signature is valid.
func verifyDiscordSignature(w http.ResponseWriter, r *http.Request, body []byte, publicKeyHex string) bool {
	signature := r.Header.Get("X-Signature-Ed25519")
	timestamp := r.Header.Get("X-Signature-Timestamp")

	sigBytes, err := hex.DecodeString(signature)
	if err != nil {
		log.Printf("Failed to decode signature hex: %v", err)
		http.Error(w, "Invalid signature format", http.StatusUnauthorized)
		return false
	}

	pubKeyBytes, err := hex.DecodeString(publicKeyHex)
	if err != nil {
		// This is an internal server error because the public key is configured, not user input.
		log.Printf("Failed to decode public key hex: %v", err)
		http.Error(w, "Invalid public key configuration", http.StatusInternalServerError)
		return false
	}

	message := append([]byte(timestamp), body...)
	if !ed25519.Verify(pubKeyBytes, message, sigBytes) {
		log.Printf("Signature verification failed. Timestamp: %s, Signature: %s", timestamp, signature)
		http.Error(w, "Signature verification failed", http.StatusUnauthorized)
		return false
	}

	return true // Signature is valid
}

func isSlashCommand(i *disgo.Interaction) bool {
	return i.Data != nil && i.Data.InteractionDataType() == disgo.FlagApplicationCommandTypeCHAT_INPUT
}

func discordInteraction(w http.ResponseWriter, r *http.Request) {
	// IMPORTANT: Replace "XXXXXXXXX" with your actual Discord application's public key.
	// It's recommended to load this from a configuration file or environment variable.
	publicKey := "XXXXXXXXX"

	body, err := io.ReadAll(r.Body)
	if err != nil {
		log.Printf("Failed to read request body: %v", err)
		http.Error(w, "Failed to read request body", http.StatusBadRequest)
		return
	}

	// Verify the signature
	if !verifyDiscordSignature(w, r, body, publicKey) {
		// Error response already sent by verifyDiscordSignature
		return
	}

	// Signature verified, proceed to process the interaction
	var interaction disgo.Interaction
	if err := json.Unmarshal(body, &interaction); err != nil {
		log.Printf("Failed to parse interaction JSON: %v", err)
		http.Error(w, "Failed to parse interaction", http.StatusBadRequest)
		return
	}

	// Handle PING interaction type for initial verification
	if interaction.Type == disgo.FlagInteractionTypePING {
		log.Println("Received PING interaction, responding with PONG.")
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(disgo.InteractionResponse{
			Type: disgo.FlagInteractionCallbackTypePONG,
		})
		return
	}

	// Handle different interaction types
	switch interaction.Type {
	case disgo.FlagInteractionTypeAPPLICATION_COMMAND:
		// Check if it's specifically a slash command (CHAT_INPUT)
		if isSlashCommand(&interaction) {
			features.HandleSlash(interaction, w)
		} else {
			// Handle other application command types if needed (e.g., User, Message commands)
			commandType := "Unknown"
			if interaction.Data != nil {
				commandType = string(interaction.Data.InteractionDataType())
			}
			log.Printf("Received non-slash application command type: %s", commandType)
			http.Error(w, "Unhandled application command type", http.StatusBadRequest)
		}
	// Add cases for other interaction types like Message Component or Modal Submit if needed
	// case disgo.FlagInteractionTypeMESSAGE_COMPONENT:
	//    handleMessageComponent(interaction, w)
	// case disgo.FlagInteractionTypeMODAL_SUBMIT:
	//    handleModalSubmit(interaction, w)
	default:
		log.Printf("Received unhandled interaction type: %d", interaction.Type)
		// It's generally better to respond with an ACK and log, rather than an error,
		// unless you know this type is invalid. Responding with an error might cause
		// Discord to show "Interaction failed".
		// Consider acknowledging the interaction if unsure:
		// ackResponse(w)
		http.Error(w, "Unhandled interaction type", http.StatusBadRequest) // Or use ackResponse(w)
	}
}

// ackResponse sends a deferred channel message update response.
// Useful for acknowledging interactions quickly before processing.
/*
func ackResponse(w http.ResponseWriter) {
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(disgo.InteractionResponse{
		Type: disgo.FlagInteractionCallbackTypeDEFERRED_CHANNEL_MESSAGE_WITH_SOURCE,
	})
}
*/
