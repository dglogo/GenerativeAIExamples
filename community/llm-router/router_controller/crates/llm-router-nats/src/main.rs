// use llm_router_nats::NatsError;
// use nats::asynk::Connection;
// use serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyRequest {
    field1: String,
    field2: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MyResponse {
    result: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to NATS server
    let nc = nats::asynk::connect("nats://demo.nats.io:4222").await?;

    // Spawn a subscriber that listens for requests
    let nc_clone = nc.clone();
    tokio::spawn(async move {
        let sub = nc_clone.subscribe("my_subject").await.unwrap();

        while let Some(msg) = sub.next().await {
            // Deserialize the request
            let request: MyRequest = serde_json::from_slice(&msg.data).unwrap();

            // Handle the request and create a response
            let response = MyResponse {
                result: format!("Received: {} and {}", request.field1, request.field2),
            };

            // Serialize the response and send it back
            let response_data = serde_json::to_vec(&response).unwrap();
            msg.respond(&response_data).await.unwrap();
        }
    });

    // Create a request
    let request = MyRequest {
        field1: "Hello".to_string(),
        field2: 42,
    };

    // Serialize the request
    let request_data = serde_json::to_vec(&request)?;

    // Send the request and wait for a response
    let response_msg = nc.request("my_subject", &request_data).await?;

    // Deserialize the response
    let response: MyResponse = serde_json::from_slice(&response_msg.data)?;

    println!("Got response: {:?}", response);

    Ok(())
}
