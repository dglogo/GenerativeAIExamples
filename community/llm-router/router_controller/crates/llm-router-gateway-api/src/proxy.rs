//! Proxy
use crate::config::RouterConfig;
use crate::error::GatewayApiError;
use crate::stream::ReqwestStreamAdapter;
use crate::triton::{InferInputTensor, InferInputs, Output};
use bytes::Bytes;
use http::request::Parts;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Method, Request, Response, Uri};
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use tokio::task;

fn print_config(config: &RouterConfig) {
    println!("{:#?}", config);
}

fn extract_forward_uri_path_and_query(req: &Request<Incoming>) -> Result<Uri, GatewayApiError> {
    let uri = req
        .uri()
        .path_and_query()
        .map(|x| x.as_str())
        .unwrap_or("")
        .to_string()
        .parse::<Uri>()?;

    Ok(uri)
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

type Messages = Vec<Message>;

fn extract_messages(value: &Value) -> Option<Messages> {
    value
        .get("messages")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

fn convert_messages_to_text_input(messages: &Messages) -> String {
    let text_input = serde_json::to_string(messages).unwrap();
    shorten_string(&text_input, 2000)
}

// fn extract_messages(value: &Value) -> Result<String, GatewayApiError> {
//     let messages = &value["messages"];
//     // let text_input = convert_messages(messages.as_array().unwrap());
//     // let text_input = shorten_string(text_input, 2000);
//     // Ok(text_input)
// }

fn shorten_string(s: &str, max_length: usize) -> String {
    let len = s.len();
    if len <= max_length {
        s.to_string()
    } else {
        s[len - max_length..].to_string()
    }
}

// fn convert_messages_to_text_input(messages: &Vec<Value>) -> String {
//     let mut output = String::new();
//     for message in messages {
//         let role = message["role"].as_str().unwrap();
//         let content = message["content"].as_str().unwrap();
//         output.push_str(&format!("{}: {}\n", role, content));
//     }
//     println!("converted messages to: {:#?}", output);
//     output
// }

fn extract_stream(parts: &Parts, value: &Value) -> Result<bool, GatewayApiError> {
    if parts.method == Method::POST
        && parts
            .headers
            .get("content-type")
            .map(|v| v.to_str().unwrap_or(""))
            == Some("application/json")
    {
        Ok(value["stream"].as_bool().unwrap_or(false))
    } else {
        Ok(false)
    }
}

async fn choose_random_model(config: &RouterConfig) -> Result<usize, GatewayApiError> {
    let n = config.clone().llms.len() - 1;
    let model_index = task::spawn_blocking(move || {
        let mut rng = rand::thread_rng();
        let model_index: usize = rng.gen_range(0..=n);
        println!("{model_index:?}");
        model_index
    })
    .await?;

    Ok(model_index)
}

// fn select_model(win_rates: &[f64], _threshold: f64) -> Option<usize> {
//     // TODO This is hardcoded for now, please see below link for more details:
//     // https://nvidia.slack.com/archives/C07ANPS4AF6/p1723568157295199?thread_ts=1723511832.899679&cid=C07ANPS4AF6
//     let threshold = 0.11;
//     let mut sorted_list: Vec<&f64> = win_rates.iter().collect();
//     sorted_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     println!("sorted_list: {:#?}", sorted_list);

//     let index = 0;
//     for (index, rate) in sorted_list.iter().skip(1).enumerate() {
//         println!("index: {:#?}, rate: {:#?}", index, rate);
//         if *rate >= &threshold {
//             return Some(index + 1);
//         }
//     }

//     Some(index)
// }

async fn choose_model(
    config: &RouterConfig,
    client: &reqwest::Client,
    text_input: &str,
    _threshold: f64,
) -> Result<usize, GatewayApiError> {
    let text_tensor = InferInputTensor {
        name: "TEXT".to_string(),
        datatype: "BYTES".to_string(),
        shape: vec![1, 1],
        data: vec![vec![text_input.to_string()]],
    };

    let data = InferInputs {
        inputs: vec![text_tensor],
    };

    let url = config.policy.url.clone();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap();
    println!("Response: {:#?}", &response);

    let response: Output = response.json().await.unwrap();
    println!("Response: {:#?}", response);

    let data = &response
        .outputs
        .first()
        .expect("missing a tensor output from triton")
        .data;
    println!("data: {:#?}", data);

    // let model_index = select_model(data, threshold).unwrap();

    let model_index = response
        .outputs
        .first()
        .expect("missing a tensor output from triton")
        .data
        .iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .expect("unable to find largest value");
    println!("model_index: {:#?}", model_index);

    Ok(model_index)
}

fn modify_model(value: Value, model: &str) -> Result<Value, GatewayApiError> {
    let mut json = value.clone();
    json["model"] = Value::String(model.to_string());
    Ok(json)
}

#[derive(Serialize, Debug)]
enum Policy {
    Manual,
    Random,
    Bert,
}

impl<'de> Deserialize<'de> for Policy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "manual" => Ok(Policy::Manual),
            "random" => Ok(Policy::Random),
            "bert" => Ok(Policy::Bert),
            _ => Err(de::Error::invalid_value(
                Unexpected::Str(&s),
                &"a valid policy",
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NimLlmRouterParams {
    policy: Policy,
    model: String,
    threshold: f64,
}

fn extract_nim_llm_router_params(value: &Value) -> Option<NimLlmRouterParams> {
    value
        .get("nim-llm-router")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

fn remove_nim_llm_router_params(mut value: Value) -> Value {
    value
        .as_object_mut()
        .map(|map| map.remove("nim-llm-router"));
    value
}

pub async fn proxy(
    req: Request<Incoming>,
    config: RouterConfig,
) -> Result<Response<BoxBody<Bytes, GatewayApiError>>, GatewayApiError> {
    // Check config
    print_config(&config);

    // Extract the forwarding URI path and query
    // let forward_uri_path_and_query = req
    let forward_uri_path_and_query = extract_forward_uri_path_and_query(&req)?;
    println!("forward_uri_path_and_query: {forward_uri_path_and_query:#?}");

    // Extract necessary parts from the original request
    let (parts, body) = req.into_parts();
    println!("parts: {parts:#?}");
    println!("body: {body:#?}");

    // Read the body
    let body_bytes = body.collect().await?.to_bytes();
    // let body_bytes = read_body(body).await?;
    println!("body_bytes: {body_bytes:#?}");

    // Create json
    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("body_str: {:#?}", &body_str);
    let json: Value = serde_json::from_str(&body_str).unwrap_or(Value::Null);
    println!("json: {:#?}", &json);

    // Check if this is a streaming request
    let is_stream = extract_stream(&parts, &json)?;
    println!("is_stream: {is_stream:#?}");

    let messages = extract_messages(&json).unwrap();
    println!("messages: {:#?}", &messages);

    let text_input = convert_messages_to_text_input(&messages);
    println!("text_input: {:#?}", &text_input);

    // Create a reqwest client
    // Wrap this in a singleton pattern. TODO
    // https://crates.io/crates/inference#:~:text=The%20idiomatic%20way%20appears%20to%20be%20storing%20a%20single%20master%20Client%20in%20a%20struct%20and%20then%20providing%20a%20function%20that%20returns%20a%20clone%20of%20the%20Client%20since%20Cloning%20clients%20is%20cheap.
    let client = reqwest::Client::new();

    // Decide how to route
    let model_index = if let Some(nim_llm_router_params) = extract_nim_llm_router_params(&json) {
        println!("nim-llm-router params: {:#?}", nim_llm_router_params);
        match &nim_llm_router_params.policy {
            Policy::Manual => {
                // If model is set, we want to use that first.
                // If model cannot be found in config, default to random
                println!("Policy: manual");
                println!("{:?}", &nim_llm_router_params.model);
                let model_index = config.get_model_index_by_model(&nim_llm_router_params.model);
                println!("model_index: {:#?}", model_index);
                let model_index = model_index.unwrap();
                println!("model_index: {:?}", model_index);
                model_index
            }
            Policy::Random => {
                println!("Policy: manual");
                choose_random_model(&config).await?
            }
            Policy::Bert => {
                println!("Policy: bert");
                choose_model(
                    &config,
                    &client,
                    &text_input,
                    nim_llm_router_params.threshold,
                )
                .await?
            } // Policy::Bert3 => {
              //     println!("Policy: bert3");
              //     choose_model(&config, &client, &text_input).await?
              // }
              // Policy::Sw => {
              //     println!("Policy: sw");
              //     choose_model(&config, &client, &text_input).await?
              // }
        }
    } else {
        // Decide at random
        println!("nim-llm-router params does NOT exist so choosing at random");
        choose_random_model(&config).await?
    };

    // Use model_index with the config to get the api_base
    let api_base = config.get_api_base_by_index(model_index).ok_or_else(|| {
        GatewayApiError::ModelNotFound(
            "Parameter api_base could not be found in the config".to_string(),
        )
    })?;

    // Use model_index with the config to get the api_key
    let api_key = config.get_api_key_by_index(model_index).ok_or_else(|| {
        GatewayApiError::ModelNotFound(
            "Parameter api_key could not be found in the config".to_string(),
        )
    })?;

    // Use model_index with the config to get the model_name
    let model = config.get_model_by_index(model_index).ok_or_else(|| {
        GatewayApiError::ModelNotFound(
            "Parameter model could not be found in the config".to_string(),
        )
    })?;
    println!("api_base: {:#?}", api_base);
    println!("api_key: {:#?}", api_key);
    println!("model: {:#?}", model);

    // Remove NIM LLM Router params from the body
    let json = remove_nim_llm_router_params(json);
    println!("json after removing nim llm router params: {:?}", json);

    // Modify the model field in the json
    let json = modify_model(json, model)?;
    println!("json after modifing model: {:#?}", &json);

    // Create the method and headers. These can also be extracted
    // from from the forward_req but for now it's fine to leave this way.
    let method = http::Method::POST;
    let mut headers = http::HeaderMap::new();

    // Add headers
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );

    // Build the reqwest request using previously modified &json
    let uri = format!("{}{}", api_base, forward_uri_path_and_query);
    let mut reqwest_request = client.request(method, uri).json(&json);

    // Insert headers - this looks like a consuming builder pattern
    // which looks odd but should be OK.
    for (name, value) in headers.iter() {
        reqwest_request = reqwest_request.header(name, value);
    }
    println!("reqwest_request: {reqwest_request:#?}");

    // Send the request to the forwarding address
    let reqwest_response = reqwest_request.send().await?.error_for_status()?;

    // Get the status and headers of the response
    let status = reqwest_response.status();
    let headers = reqwest_response.headers().clone();

    // Handle the response dependent on if streaming or not
    if is_stream {
        // Create our custom adapter
        let stream = reqwest_response.bytes_stream();
        let body = ReqwestStreamAdapter {
            inner: Box::pin(stream),
        };

        // Wrap the ReqwestStreamAdapter in a BoxBody
        let boxed_body = BoxBody::new(body);

        // Create the hyper Response with our boxed body
        let mut client_res = Response::new(boxed_body);

        *client_res.status_mut() = status;
        *client_res.headers_mut() = headers;

        Ok(client_res)
    } else {
        // Handle non-streaming response
        let body_bytes = reqwest_response.bytes().await?;

        let body = Full::from(body_bytes)
            .map_err(|never| match never {})
            .boxed();

        let mut client_res = Response::builder().status(status).body(body)?;

        // Copy all headers from the forwarded response
        *client_res.headers_mut() = headers;

        println!("client_res: {client_res:#?}");

        Ok(client_res)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Llm, Policy};

    use super::*;
    // use http::Method;
    // use serde_json::json;

    // #[tokio::test]
    // async fn test_convert_messages() {
    //     let messages = vec![
    //         json!({"role": "user", "content": "Hello"}),
    //         json!({"role": "assistant", "content": "Hi there!"}),
    //     ];
    //     let expected = "user: Hello\nassistant: Hi there!\n";
    //     assert_eq!(convert_messages(&messages), expected);
    // }

    // #[tokio::test]
    // async fn test_convert_messages_empty() {
    //     let messages: Vec<Value> = vec![];
    //     let expected = "";
    //     assert_eq!(convert_messages(&messages), expected);
    // }

    // #[tokio::test]
    // fn test_convert_messages_missing_role() {
    //     let messages = vec![json!({"content": "Hello"})];
    //     assert!(convert_messages(&messages).is_err());
    // }

    // #[tokio::test]
    // async fn test_convert_messages_missing_content() {
    //     let messages = vec![json!({"role": "user"})];
    //     assert!(convert_messages(&messages).is_err());
    // }

    #[tokio::test]
    async fn test_shorten_string() {
        let s = "Hello, world!".to_string();
        let max_length = 5;
        let expected = "orld!".to_string();
        assert_eq!(shorten_string(&s, max_length), expected);
    }

    #[tokio::test]
    async fn test_shorten_string_empty() {
        let s = "".to_string();
        let max_length = 5;
        let expected = "".to_string();
        assert_eq!(shorten_string(&s, max_length), expected);
    }

    #[tokio::test]
    async fn test_shorten_string_longer() {
        let s = "Hello, world!".to_string();
        let max_length = 15;
        let expected = "Hello, world!".to_string();
        assert_eq!(shorten_string(&s, max_length), expected);
    }

    #[tokio::test]
    async fn test_shorten_string_equal() {
        let s = "Hello, world!".to_string();
        let max_length = 13;
        let expected = "Hello, world!".to_string();
        assert_eq!(shorten_string(&s, max_length), expected);
    }

    #[tokio::test]
    async fn test_proxy_success() {
        let policy = Policy {
            url: "http://0.0.0.0:8000/v2/models/bert_ensemble/infer".to_string(),
        };
        let llama = Llm {
            name: "llama3".to_string(),
            api_base: "http://0.0.0.0:8000".to_string(),
            api_key: "".to_string(),
            model: "meta/llama3-8b-instruct".to_string(),
        };
        let mistral = Llm {
            name: "mistral".to_string(),
            api_base: "http://0.0.0.0:8001".to_string(),
            api_key: "".to_string(),
            model: "mistralai/mistral-7b-instruct-v0.3".to_string(),
        };
        let openai = Llm {
            name: "openai".to_string(),
            api_base: "https://api.openai.com".to_string(),
            api_key: "".to_string(),
            model: "gpt-4o".to_string(),
        };
        let ngc = Llm {
            name: "ngc".to_string(),
            api_base: "https://integrate.api.nvidia.com".to_string(),
            api_key: "".to_string(),
            model: "meta/llama-3.1-405b-instruct".to_string(),
        };
        let _config = RouterConfig {
            policy,
            llms: vec![llama, mistral, openai, ngc],
        };

        // let req = reqwest::Request::new(Method::POST, "https://example.com/forward".to_string())
        //     .header("Content-Type", "application/json")
        //     .body(hyper::body::Body::from(r#"{"input": "hello world"}"#));

        // let res = proxy(req, config).await.unwrap();

        // assert_eq!(res.status(), 200);
        // assert_eq!(res.headers().get("Content-Type"), Some("application/json"));
    }

    // #[tokio::test]
    // async fn test_proxy_streaming_request() {
    //     let config = RouterConfig {
    //         // setup config
    //         api_bases: vec!["https://api.example.com".to_string()],
    //         api_keys: vec!["api_key".to_string()],
    //         models: vec!["model_name".to_string()],
    //     };

    //     let req = Request::new(Method::POST, "https://example.com/forward")
    //         .header("Content-Type", "application/json")
    //         .body(Body::from(r#"{"input": "hello world"}"#));

    //     let res = proxy(req, config).await.unwrap();

    //     assert_eq!(res.status(), 200);
    //     assert_eq!(res.headers().get("Content-Type"), Some("application/json"));
    //     assert!(res.body().is_streaming());
    // }

    // #[tokio::test]
    // async fn test_proxy_model_not_found() {
    //     let config = RouterConfig {
    //         // setup config
    //         api_bases: vec![],
    //         api_keys: vec![],
    //         models: vec![],
    //     };

    //     let req = Request::new(Method::POST, "https://example.com/forward")
    //         .header("Content-Type", "application/json")
    //         .body(Body::from(r#"{"input": "hello world"}"#));

    //     let res = proxy(req, config).await;

    //     assert!(res.is_err());
    //     assert_eq!(res.err().unwrap().to_string(), "Model not found");
    // }

    // #[tokio::test]
    // async fn test_proxy_invalid_request() {
    //     let config = RouterConfig {
    //         // setup config
    //         api_bases: vec!["https://api.example.com".to_string()],
    //         api_keys: vec!["api_key".to_string()],
    //         models: vec!["model_name".to_string()],
    //     };

    //     let req = Request::new(Method::POST, "https://example.com/forward")
    //         .header("Content-Type", "application/json")
    //         .body(Body::from("invalid json"));

    //     let res = proxy(req, config).await;

    //     assert!(res.is_err());
    //     assert_eq!(res.err().unwrap().to_string(), "Invalid request");
    // }
}
