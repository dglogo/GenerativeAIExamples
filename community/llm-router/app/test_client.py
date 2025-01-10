import os
from openai import OpenAI
from app_info import *

# client = OpenAI()

client = OpenAI(
    # This is the default and can be omitted
    api_key="foobar",
)

# client.api_key = ""
client.base_url = router_controller_url

messages = [
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
    "content":"Can you write me a song?  Use as many emojis as possible."
    }
]

print("-" * 79)
print("Unary")
print("-" * 79)
chat_completion = client.chat.completions.create(
    messages=messages,
    model="",
)

print(chat_completion.choices[0].message.content)

print("-" * 79)
print("Streaming")
print("-" * 79)
stream = client.chat.completions.create(
    model="",
    messages=messages,
    stream=True,
)
for chunk in stream:
    print(chunk.choices[0].delta.content or "", end="")
