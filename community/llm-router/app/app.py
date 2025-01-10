import gradio as gr
from css.css import css, theme
from llm import LLMClient
import os
import logging
import sys
import os
from app_info import *


logging.basicConfig(
    level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s", datefmt="%Y-%m-%d %H:%M:%S",
)

client = LLMClient(api_key=openai_api_key, base_url=router_controller_url)

def policy_dropdown_function(choice):
    return f"Selected Policy: {choice}"

def model_dropdown_function(choice):
    return f"Selected Model: {choice}"

def slider_function(value):
    return f"Slider value: {value}"

def list_policies():
    print(policies_list)
    return policies_list

def list_models():
    return models_list

LONG_EXAMPLE = """Q: Premise: "Men are standing around produce at an open market."
Hypothesis: "Two men are shopping for produce."
Is the hypothesis entailed by the premise?
Options:
- yes
- it is not possible to tell
- no
A: Mean standing around produce implies that the men are shopping for produce.
The answer is yes.

QUESTION: Given the sentence "A young girl with dark hair bends over a piece of pottery that is on a table." is it true that "A young girl is near a table."?

Let's solve it slowly: If a girl sees something on a table then she is near a table.
The answer is yes.

[QUESTION] Premise: "A clown on a bicycle wearing face paint."
Hypothesis: "There is a jocker making his look more funny with face paint."
Do we know that the hypothesis entailed by the premise?
A clown would be making his look more funny with face paint.
The answer is yes.

Q: Given the sentence "A man wearing a long-sleeved gray shirt and dark pants is walking through a gray stone archway." is it true that "There is an archway."?
A: If you are walking through an archway then there must be an archway.
The answer is yes.

QUESTION: Premise: "Three men are visiting at a dressy gathering."
Hypothesis: "The men are wearing fancy tuxedos."
Is the hypothesis entailed by the premise?
Options:
- yes
- it is not possible to tell
- no

Let's solve it slowly: The men don't need to be at a gathering if they are wearing fancy tuxedos.
The answer is it is not possible to tell.

QUESTION: Premise: "Group of about a dozen people at a beach with someone sitting at an assembled booth."
Based on this premise, can we conclude that the hypothesis "Probably gathered for the pipe challenge." is true?
Options:
- yes
- it is not possible to tell
- no

Let's solve it slowly"""

examples = [
    [
        "What is CUDA?",
        "manual",
        "meta/llama3-8b-instruct",
        # 0.2,
    ],
    [
        "What is Triton Inference Server?",
        "manual",
        "mistralai/mistral-7b-instruct-v0.3",
        # 0.2,
    ],
    [
        "What is NVIDIA Metropolis?",
        "manual",
        "meta/llama-3.1-405b-instruct",
        # 0.2,
    ],
    [
        "Write a poem about any random topic.",
        "random",
        "",
        # 0.2,
    ],
    [
        "Hello!",
        "bert",
        "",
        # 0.2,
    ],
    [
        "What is the capital of France?",
        "bert",
        "",
        # 0.2,
    ],
    [
        "Write a company safety plan for sic code 3245 in Phoenix Arizona.",
        "bert",
        "",
        # 0.2,
    ],
    [
        "we are planning a content migration. we would like a list of terms to identify the status of the content and if the content needs to be migrated. can you propose a list of terms?",
        "bert",
        "",
        # 0.2,
    ],
    [
        LONG_EXAMPLE,
        "bert",
        "",
        # 0.2,
    ],
]


chatbot = gr.Chatbot(label="LLM Router", elem_id="chatbot", show_copy_button=True)

with gr.Blocks(theme=theme, css=css) as chat:
    with gr.Row():
        policy_dropdown = gr.Dropdown(
            choices=list_policies(),
            label="Select the Policy",
            min_width=50,
            scale=1
        )
        model_dropdown = gr.Dropdown(
            choices=list_models(),
            label='Select the model if Policy is "manual"',
            min_width=50,
            scale=1
        )
        # slider = gr.Slider(
        #     minimum=0.0,
        #     maximum=1.0,
        #     step=0.01,
        #     label="Threshold",
        #     value=0.05,
        #     scale=1
        # )
    
    dropdown_output = gr.Textbox(label="Policy Option", visible= False)
    model_output = gr.Textbox(label="Model Option", visible= False)
    # slider_output = gr.Textbox(label="Slider Value", visible=False)
    
    policy_dropdown.change(fn=policy_dropdown_function, inputs=[policy_dropdown], outputs=[dropdown_output])
    model_dropdown.change(fn=model_dropdown_function, inputs=[model_dropdown], outputs=[dropdown_output])
    # slider.change(fn=slider_function, inputs=[slider], outputs=[slider_output])

    chat_interface = gr.ChatInterface(
        fn=client.predict,
        chatbot=chatbot,
        # additional_inputs=[policy_dropdown, model_dropdown, slider],
        additional_inputs=[policy_dropdown, model_dropdown],
        title="NVIDIA LLM Router",
        stop_btn=None,
        retry_btn=None,
        undo_btn=None,
        clear_btn="Clear Chat History",
        autofocus=True,
        fill_height=True,
        examples=examples,
    )

    # chat_interface.render()

if __name__ == "__main__":
    chat.queue().launch(
        share=False,
        favicon_path="/app/css/faviconV2.png",
        allowed_paths=[
            "/app/fonts/NVIDIASansWebWOFFFontFiles/WOFF2/NVIDIASans_W_Rg.woff2",
            "/app/fonts/NVIDIASansWebWOFFFontFiles/WOFF2/NVIDIASans_W_Bd.woff2",
            "/app/fonts/NVIDIASansWebWOFFFontFiles/WOFF2/NVIDIASans_W_It.woff2",
        ],
    )
