#![allow(unused)] // For beginning only.

// use async_openai::{
//     config::OpenAIConfig,
//     types::{
//         ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
//         ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
//         CreateCompletionRequestArgs,
//     },
//     Client,
// };
// use std::env;
// use std::error::Error;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Create client
//     let url = "http://0.0.0.0/v1";
//     // let url = "http://10.137.197.23:8000/v1";
//     // let config = OpenAIConfig::new()
//     //     .with_api_base(url);
//     let key = "NGC_API_KEY";
//     let api_key = env::var(key).unwrap_or_else(|_| String::from(""));

//     let config = OpenAIConfig::new().with_api_base(url).with_api_key(api_key);
//     println!("{:#?}", &config);
//     let client = Client::with_config(config);

//     // Create request using builder pattern
//     // Every request struct has companion builder struct with same name + Args suffix
//     // let model_id = "mistralai/mistral-7b-instruct-v0.3";
//     // let model_id = "mistralai/Mistral-7B-Instruct-v0.3";
//     // let model_id = "meta/llama-3.1-405b-instruct";
//     // let model_id = "nv-mistralai/mistral-nemo-12b-instruct";
//     let model_id = "router";
//     let max_tokens: u32 = 256;
//     let request = CreateChatCompletionRequestArgs::default()
//         .model(model_id)
//         .max_tokens(max_tokens)
//         .messages([
//             ChatCompletionRequestSystemMessageArgs::default()
//                 .content("You are a helpful assistant.")
//                 .build()?
//                 .into(),
//             ChatCompletionRequestUserMessageArgs::default()
//                 .content("Who won the world series in 2020?")
//                 .build()?
//                 .into(),
//             ChatCompletionRequestAssistantMessageArgs::default()
//                 .content("The Los Angeles Dodgers won the World Series in 2020.")
//                 .build()?
//                 .into(),
//             ChatCompletionRequestUserMessageArgs::default()
//                 .content("Where was it played?")
//                 .build()?
//                 .into(),
//         ])
//         .build()?;
//     println!("{:#?}", &request);

//     println!("{}", serde_json::to_string(&request).unwrap());

//     let response = client.chat().create(request).await?;

//     println!("\nResponse:\n");
//     for choice in response.choices {
//         println!(
//             "{}: Role: {}  Content: {:?}",
//             choice.index, choice.message.role, choice.message.content
//         );
//     }

//     Ok(())
// }

use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://0.0.0.0:8084/api/v1/process")
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&json!({"text": "Hello"}))
        .send()
        .await?;

    println!("{}", response.text().await?);
    Ok(())
}
