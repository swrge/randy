package http

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"
)

// ContentTypes for HTTP requests
const (
	ContentTypeJSON = "application/json"
)

// Client represents a simplified HTTP client for making requests to bot-requester
type Client struct {
	// BaseURL is the base URL for the bot-requester service
	BaseURL string
	// HTTPClient is the underlying HTTP client
	HTTPClient *http.Client
}

// NewClient creates a new HTTP client for communicating with bot-requester
func NewClient() *Client {
	// Get bot-requester URL from environment
	requesterURL := os.Getenv("BOT_REQUESTER_URL")
	if requesterURL == "" {
		requesterURL = "http://localhost:8088" // Default fallback
	}

	return &Client{
		BaseURL: requesterURL,
		HTTPClient: &http.Client{
			Timeout: 10 * time.Second,
		},
	}
}

// SendMessage sends a message to the specified channel
func (c *Client) SendMessage(channelID string, message map[string]interface{}) ([]byte, error) {
	endpoint := fmt.Sprintf("%s/api/v10/channels/%s/messages", c.BaseURL, channelID)
	return c.SendRequest("POST", endpoint, message)
}

// EditMessage edits a message in the specified channel
func (c *Client) EditMessage(channelID, messageID string, message map[string]interface{}) ([]byte, error) {
	endpoint := fmt.Sprintf("%s/api/v10/channels/%s/messages/%s", c.BaseURL, channelID, messageID)
	return c.SendRequest("PATCH", endpoint, message)
}

// DeleteMessage deletes a message in the specified channel
func (c *Client) DeleteMessage(channelID, messageID string) error {
	endpoint := fmt.Sprintf("%s/api/v10/channels/%s/messages/%s", c.BaseURL, channelID, messageID)
	_, err := c.SendRequest("DELETE", endpoint, nil)
	return err
}

// CreateReaction adds a reaction to a message
func (c *Client) CreateReaction(channelID, messageID, emoji string) error {
	endpoint := fmt.Sprintf("%s/api/v10/channels/%s/messages/%s/reactions/%s/@me", 
		c.BaseURL, channelID, messageID, emoji)
	_, err := c.SendRequest("PUT", endpoint, nil)
	return err
}

// SendRequest makes an HTTP request to bot-requester
func (c *Client) SendRequest(method, endpoint string, payload interface{}) ([]byte, error) {
	var reqBody []byte
	var err error

	// Prepare request body if payload is not nil
	if payload != nil {
		reqBody, err = json.Marshal(payload)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal request payload: %w", err)
		}
	}

	// Create the HTTP request
	req, err := http.NewRequest(method, endpoint, bytes.NewBuffer(reqBody))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	// Set headers
	req.Header.Set("Content-Type", ContentTypeJSON)

	// Log request details
	log.Printf("Sending %s request to %s", method, endpoint)
	if len(reqBody) > 0 && len(reqBody) < 500 {
		log.Printf("Request body: %s", string(reqBody))
	}

	// Send the request
	resp, err := c.HTTPClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("request failed: %w", err)
	}
	defer resp.Body.Close()

	// Read the response body
	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %w", err)
	}

	// Check if the request was successful
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, fmt.Errorf("unexpected status code: %d, body: %s", 
			resp.StatusCode, string(respBody))
	}

	// Log response details
	log.Printf("Received response with status code %d", resp.StatusCode)
	if len(respBody) > 0 && len(respBody) < 500 {
		log.Printf("Response body: %s", string(respBody))
	}

	return respBody, nil
}