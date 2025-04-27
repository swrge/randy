#!/bin/bash

# This script is an minimalist IPAM for ISC KEA using REST, kea-dhcp4, kea-agent and charmbracelet's gum utility.
#

# command to get all ip reservations:
# jq '.[0].arguments.Dhcp4.reservations[]."ip-address", .[0].arguments.Dhcp4.subnet4[].reservations?[]?."ip-address"' result.json
# jq '[ (.[0].arguments.Dhcp4.reservations[] | [.hostname, ."hw-address", ."ip-address"]), (.[0].arguments.Dhcp4.subnet4[].reservations?[]? | [.hostname, ."hw-address", ."ip-address"]) ]' result.json

# command to convert json to csv:
# jq -r '( ["hostname","hw-address","ip-address"], (.[0].arguments.Dhcp4.reservations[] | [.hostname, ."hw-address", ."ip-address"]), (.[0].arguments.Dhcp4.subnet4[].reservations?[]? | [.hostname, ."hw-address", ."ip-address"]) ) | @csv' result.json



LOCKFILE="/tmp/ipam.lock"
exec 200 > "$LOCKFILE"
if ! flock -n 200; then
    echo "Un autre utilisateur est en train d'utiliser l'IPAM. Veuillez réessayer plus tard."
    exit 1
fi
