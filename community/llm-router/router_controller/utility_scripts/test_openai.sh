#!/bin/bash

curl -X 'POST' \
  'https://api.openai.com/v1/chat/completions' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4o",
    "messages": [
      {
        "role":"user",
        "content":"Hello! How are you?"
      }
    ],
    "max_tokens": 32,
    "stream": false
  }' | jq

# curl -X 'POST' \
#   'https://api.openai.com/v1/chat/completions' \
#   -H 'accept: application/json' \
#   -H 'Content-Type: application/json' \
#   -H "Authorization: Bearer $OPENAI_API_KEY" \
#   -d '{
#     "model": "gpt-4o",
#     "messages": [
#       {
#         "role":"user",
#         "content":"Hello! How are you?"
#       },
#       {
#         "role":"assistant",
#         "content":"Hi! I am quite well, how can I help you today?"
#       },
#       {
#         "role":"user",
#         "content":"Can you write me a song?"
#       }
#     ],
#     "max_tokens": 32,
#     "stream": false
#   }' | jq
