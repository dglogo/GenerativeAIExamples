#!/bin/bash

docker run \
  -u $(id -u) \
  --gpus '"device=2"' \
  -e "NGC_API_KEY=$NGC_CLI_API_KEY" \
  -p "8001:8000" \
  -v /home/user/.cache/nim:/opt/nim/.cache \
  nvcr.io/nim/mistralai/mistral-7b-instruct-v03:1.0.0
