#!/bin/bash

curl -X 'POST' \
  'http://0.0.0.0:8001/v1/chat/completions' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "model": "mistralai/mistral-7b-instruct-v0.3",
    "messages": [
      {
        "role":"user",
        "content":"Hello! How are you?"
      },
      {
        "role":"assistant",
        "content":"Hi! I am quite well, how can I help you today?"
      },
      {
        "role":"user",
        "content":"Can you write me a song?"
      }
    ],
    "max_tokens": 32,
    "stream": true
  }'
