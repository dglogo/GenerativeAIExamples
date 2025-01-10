
import os


#endpoint models that the router can send queries to
models_list = ["meta/llama3.1-8b-instruct",
            "mistralai/mixtral-8x22b-instruct-v0.1", 
             "meta/llama-3.1-70b-instruct"]


# all routers that are actively deployed on the router controller that can be selected
# manual and random routers are deployed by default
policies_list = ["manual", "random", "task"]

#OpenAI Key for client, not required for demo
openai_api_key = "some-api-key"

# URL where the router_controller is deployed
# [Note]depending on your machine, you may have to use a global IP address
router_controller_url = "http://127.0.0.1:8084/v1"
