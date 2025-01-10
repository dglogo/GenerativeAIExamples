#!/bin/bash

mkdir -p ./results/data
mkdir -p ./results/images

docker run \
  --net host \
  -v ./results:/workspace/artifacts \
  nvcr.io/nvidia/tritonserver:24.04-py3-sdk \
  genai-perf \
  --model "meta/llama3-8b-instruct" \
  --endpoint "v1/chat/completions" \
  --endpoint-type chat \
  --service-kind openai \
  --streaming \
  --url "http://localhost:8084" \
  --num-prompts 100 \
  --prompt-source synthetic \
  --synthetic-input-tokens-mean 1024 \
  --synthetic-input-tokens-stddev 50 \
  --concurrency 10 \
  --extra-inputs "max_tokens:512" \
  --extra-input "ignore_eos:true"
