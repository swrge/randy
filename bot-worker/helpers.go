package function

import (
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/switchupcb/disgo"
)

// EnvironmentVariable retrieves an environment variable with a fallback value
func EnvironmentVariable(key, fallback string) string {
	value := os.Getenv(key)
	if value == "" {
		return fallback
	}
	return value
}

// RespondWithMessage sends a simple text response to a Discord interaction
func RespondWithMessage(w http.ResponseWriter, interaction disgo.Interaction, content string, ephemeral bool) error {
	response := &disgo.CreateInteractionResponse{
		InteractionID:    interaction.ID,
		InteractionToken: interaction.Token,
		InteractionResponse: &disgo.InteractionResponse{
			Type: disgo.FlagInteractionCallbackTypeCHANNEL_MESSAGE_WITH_SOURCE,
			Data: &disgo.Messages{
				Content: disgo.Pointer(content),
			},
		},
	}

	// Make the message ephemeral if requested
	if ephemeral && response.InteractionResponse.Data != nil {
		response.InteractionResponse.Data.Flags = disgo.Pointer(disgo.FlagMessageEPHEMERAL)
	}

	responseJSON, err := json.Marshal(response)
	if err != nil {
		log.Printf("Error marshaling response: %v", err)
		return err
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	_, err = w.Write(responseJSON)
	return err
}

// DeferResponse sends a deferred response to Discord
// This is useful when the operation might take longer than 3 seconds
func DeferResponse(w http.ResponseWriter, interaction disgo.Interaction, ephemeral bool) error {
	response := &disgo.CreateInteractionResponse{
		InteractionID:    interaction.ID,
		InteractionToken: interaction.Token,
		InteractionResponse: &disgo.InteractionResponse{
			Type: disgo.FlagInteractionCallbackTypeDEFERRED_CHANNEL_MESSAGE_WITH_SOURCE,
		},
	}

	// If ephemeral is requested, we need to include a data field with the flag
	if ephemeral {
		response.InteractionResponse.Data = &disgo.Messages{
			Flags: disgo.Pointer(disgo.FlagMessageEPHEMERAL),
		}
	}

	responseJSON, err := json.Marshal(response)
	if err != nil {
		log.Printf("Error marshaling deferred response: %v", err)
		return err
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	_, err = w.Write(responseJSON)
	return err
}

// GetOptionValue retrieves an option value from a slash command
// Returns the value and a boolean indicating if the option was found
func GetOptionValue(options []disgo.ApplicationCommandInteractionDataOption, name string) (interface{}, bool) {
	for _, option := range options {
		if option.Name == name {
			return option.Value, true
		}
	}
	return nil, false
}

// GetStringOption retrieves a string option from a slash command
// Returns the value and a boolean indicating if the option was found
func GetStringOption(options []disgo.ApplicationCommandInteractionDataOption, name string) (string, bool) {
	value, found := GetOptionValue(options, name)
	if !found {
		return "", false
	}
	
	str, ok := value.(string)
	return str, ok
}

// GetIntOption retrieves an integer option from a slash command
// Returns the value and a boolean indicating if the option was found
func GetIntOption(options []disgo.ApplicationCommandInteractionDataOption, name string) (int, bool) {
	value, found := GetOptionValue(options, name)
	if !found {
		return 0, false
	}
	
	numFloat, ok := value.(float64)
	return int(numFloat), ok
}

// GetBoolOption retrieves a boolean option from a slash command
// Returns the value and a boolean indicating if the option was found
func GetBoolOption(options []disgo.ApplicationCommandInteractionDataOption, name string) (bool, bool) {
	value, found := GetOptionValue(options, name)
	if !found {
		return false, false
	}
	
	boolVal, ok := value.(bool)
	return boolVal, ok
}