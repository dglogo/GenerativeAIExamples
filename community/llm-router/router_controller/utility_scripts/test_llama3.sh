#!/bin/bash

curl -X 'POST' \
  'http://0.0.0.0:8000/v1/chat/completions' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "model": "meta/llama3-8b-instruct",
    "messages": [
      {
        "role":"system",
        "content":"You are a helpful chatbot."
      },
      {
        "role":"assistant",
        "content":"How can I help you today?"
      },
      {
        "role":"user",
        "content":"Can you write me a song?"
      }
    ],
    "max_tokens": 32,
    "stream": false,
    "nvext": {}
  }'  | jq
