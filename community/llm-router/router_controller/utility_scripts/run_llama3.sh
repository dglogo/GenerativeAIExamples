#!/bin/bash

docker run \
  -u $(id -u) \
  --gpus '"device=1"' \
  -e "NGC_API_KEY=$NGC_CLI_API_KEY" \
  -p "8000:8000" \
  -v /home/user/.cache/nim:/opt/nim/.cache \
  nvcr.io/nim/meta/llama3-8b-instruct:1.0.0
