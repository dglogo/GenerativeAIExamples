#!/bin/bash

curl -X 'POST' \
  'https://integrate.api.nvidia.com/v1/chat/completions' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $NGC_API_KEY" \
  -d '{
    "model": "mistralai/mixtral-8x22b-instruct-v0.1",
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
    "stream": false
  }'
