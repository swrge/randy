package handler

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/switchupcb/disgo"
)

func Pong(w http.ResponseWriter, r *http.Request) {
	log.Println("Received PING interaction, responding with PONG.")
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(disgo.InteractionResponse{
		Type: disgo.FlagInteractionCallbackTypePONG,
	})
}


func HandleCommand(w http.ResponseWriter, r *http.Request, interaction *disgo.Interaction) {
	//
}

func HandleComponent(w http.ResponseWriter, r *http.Request, interaction *disgo.Interaction) {
	//
}
