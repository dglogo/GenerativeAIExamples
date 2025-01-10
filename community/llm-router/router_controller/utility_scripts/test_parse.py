import json


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

def preprocess_messages(messages: list[dict[str, str]]) -> str:
    return json.dumps(messages)

output = preprocess_messages(messages)

print(output)
