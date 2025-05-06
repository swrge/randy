package function

import (
	"bytes"
	"context"
	"crypto/ed25519"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"log/slog"
	"net/http"
	"os"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/switchupcb/disgo"
	"github.com/swrge/bot-worker/handler"
)

func init() {
	functions.HTTP("DiscordBot", DiscordBot)
}

func VerifyInteraction(r *http.Request, key ed25519.PublicKey) bool {
	var msg bytes.Buffer

	signature := r.Header.Get("X-Signature-Ed25519")
	if signature == "" {
		return false
	}

	sig, err := hex.DecodeString(signature)
	if err != nil {
		return false
	}

	if len(sig) != ed25519.SignatureSize {
		return false
	}

	timestamp := r.Header.Get("X-Signature-Timestamp")
	if timestamp == "" {
		return false
	}

	msg.WriteString(timestamp)

	defer r.Body.Close()
	var body bytes.Buffer

	// at the end of the function, copy the original body back into the request
	defer func() {
		r.Body = io.NopCloser(&body)
	}()

	// copy body into buffers
	_, err = io.Copy(&msg, io.TeeReader(r.Body, &body))
	if err != nil {
		return false
	}

	return ed25519.Verify(key, msg.Bytes(), sig)
}

func isSlashCommand(i *disgo.Interaction) bool {
	return i.Data != nil && i.Data.InteractionDataType() == disgo.FlagApplicationCommandTypeCHAT_INPUT
}

func DiscordBot(w http.ResponseWriter, r *http.Request) {
	_ = context.Background()

	body, err := io.ReadAll(r.Body)
	if err != nil {
		log.Printf("Failed to read request body: %v", err)
		http.Error(w, "Failed to read request body", http.StatusBadRequest)
		return
	}

	// Verify the signature
	if !verifySignature(w, r) {
		// Error response already sent by verifyDiscordSignature
		return
	}

	// Process interaction
	var interaction disgo.Interaction
	if err := json.Unmarshal(body, &interaction); err != nil {
		log.Printf("Failed to parse interaction JSON: %v", err)
		http.Error(w, "Failed to parse interaction", http.StatusBadRequest)
		return
	}

	// Handle different interaction types
	switch interaction.Type {
	case disgo.FlagInteractionTypePING:
		handler.Pong(w, r)
	case disgo.FlagInteractionTypeAPPLICATION_COMMAND:
		handler.HandleCommand(w, r, &interaction)
	case disgo.FlagInteractionTypeMESSAGE_COMPONENT:
		handler.HandleComponent(w, r, &interaction)
	default:
		log.Printf("Unsupported interaction type: %d", interaction.Type)
		http.Error(w, "Unsupported interaction type", http.StatusBadRequest)
		return
	}
}

func verifySignature(w http.ResponseWriter, r *http.Request) bool {
	pubkey, err := hex.DecodeString(os.Getenv("DISCORD_PUBLIC_KEY"))
	if err != nil {
		slog.Error(
			"Error decoding public key",
			"err", err,
		)

		w.WriteHeader(http.StatusInternalServerError)
		fmt.Fprint(w, "Internal Server Error due to decording public key")
		return false
	}

	if !VerifyInteraction(r, pubkey) {
		slog.Info(
			"Unauthorized request",
			"err", err,
		)

		w.WriteHeader(http.StatusUnauthorized)
		fmt.Fprint(w, "Unauthorized")
		return false
	}

	return true
}
