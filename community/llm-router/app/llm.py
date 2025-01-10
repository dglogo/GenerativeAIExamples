import logging
from openai import OpenAI


logging.basicConfig(
    level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s", datefmt="%Y-%m-%d %H:%M:%S",
)

class LLMClient:
    def __init__(self, api_key, base_url):
        self.client = OpenAI(api_key=api_key, max_retries=0,)
        self.client.base_url = base_url

    def predict(self, message, history, policy, selected_model):#, threshold):
        logging.info("start of predict()")
        logging.info(f"message: {message}")
        logging.info(f"history: {history}")
        history_openai_format = []
        for human, assistant in history:
            # remove model name from history
            assistant = assistant.split("] ", maxsplit=1)[1]

            history_openai_format.append({"role": "user", "content": human})
            history_openai_format.append({"role": "assistant", "content": assistant})

        history_openai_format.append({"role": "user", "content": f"{message}"})
        logging.info(history_openai_format)

        extra_body = {"nim-llm-router": {"model": selected_model, "policy": policy, "threshold": 0.2}}
        logging.info(extra_body)


        logging.info("self.client.chat.completions.create")

        response = self.client.chat.completions.create(
            model="",
            messages=history_openai_format,
            temperature=0.5,
            top_p=1,
            max_tokens=1024,
            stream=True,
            extra_body=extra_body,
        )

        partial_message = ""
        model_that_was_used = None
        for chunk in response:
            if chunk.choices[0].delta.content is not None:
                if model_that_was_used is None:
                    logging.info(f"setting model from {model_that_was_used} to: {chunk.model}")
                    model_that_was_used = chunk.model
                # logging.info(f"model: {model}")
                partial_message = partial_message + chunk.choices[0].delta.content
                output = f"[{model_that_was_used}] " + partial_message
                # logging.info(f"output: {output}")
                yield output
