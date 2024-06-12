package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"math/rand/v2"
	"net/http"
)

type Channel struct {
	Id       string `json:"id"`
	IsMember bool   `json:"is_member"`
	Name     string `json:"name"`
}

type ConversationsList struct {
	Channels []Channel `json:"channels"`
}

type PostMessageRequest struct {
	Channel string `json:"channel"`
	Text    string `json:"text"`
}

type JoinConversationRequest struct {
	Channel string `json:"channel"`
}

func doRequest(req *http.Request) (*http.Response, error) {
	req.Header.Add("Authorization", "")
	client := http.Client{}
	return client.Do(req)
}

func getChannels() []Channel {
	url := "https://slack.com/api/conversations.list"
	request, _ := http.NewRequest("GET", url, nil)
	resp, err := doRequest(request)
	if err != nil {
		log.Fatal(err)
	}

	defer resp.Body.Close()
	respJson := ConversationsList{}
	if err := json.NewDecoder(resp.Body).Decode(&respJson); err != nil {
		log.Fatal(err)
	}

	return respJson.Channels
}

func sendMsg(channel Channel) {
	url := "https://slack.com/api/chat.postMessage"
	msg := PostMessageRequest{
		Channel: channel.Id,
		Text:    "hi",
	}
	var body bytes.Buffer
	if err := json.NewEncoder(&body).Encode(&msg); err != nil {
		log.Fatal(err)
	}
	request, _ := http.NewRequest("POST", url, &body)
	request.Header.Add("Content-Type", "application/json; charset=utf-8")

	_, err := doRequest(request)
	if err != nil {
		log.Fatal(err)
	}
}

func join(channel Channel) {
	url := "https://slack.com/api/conversations.join"
	msg := JoinConversationRequest{
		Channel: channel.Id,
	}
	var body bytes.Buffer
	if err := json.NewEncoder(&body).Encode(&msg); err != nil {
		log.Fatal(err)
	}
	request, _ := http.NewRequest("POST", url, &body)
	request.Header.Add("Content-Type", "application/json; charset=utf-8")

	resp, err := doRequest(request)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

}

func joinAndSend(channel Channel) {
	if !channel.IsMember {
		join(channel)
	}

	sendMsg(channel)
}

func main() {
	channels := getChannels()
	fmt.Println(channels)

	for i := 0; i < 5; i++ {
		i := rand.IntN(len(channels))
		selectedChannel := channels[i]
		joinAndSend(selectedChannel)
	}
}
