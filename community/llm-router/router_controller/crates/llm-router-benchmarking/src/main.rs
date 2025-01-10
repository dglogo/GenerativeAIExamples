#![allow(dead_code)]

use goose::prelude::*;
use serde::{Deserialize, Serialize};
// use nim_api::protocols::openai::StreamingDelta;
// use nim_http_client::openai::OpenAIClient;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

// Use a vector to hold multiple messages
type Messages = Vec<Message>;

async fn loadtest_task(user: &mut GooseUser) -> TransactionResult {
    // Create instances of the Message struct to form a vector
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful chatbot.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "Can you help me find a polo?".to_string(),
        },
    ];

    // Serialize the vector of Message structs to a JSON string
    let messages_json = &serde_json::json!(messages);

    let _response = user.post_json("/chat", &messages_json).await?;

    Ok(())
}

// pub struct Messages {
//     pub message:
// }

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions").register_transaction(transaction!(loadtest_task)),
        )
        .execute()
        .await?;

    Ok(())
}
