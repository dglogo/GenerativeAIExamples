//! Config
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterConfig {
    pub policy: Policy,
    pub llms: Vec<Llm>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Policy {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Llm {
    pub name: String,
    pub api_base: String,
    pub api_key: String,
    pub model: String,
}

impl RouterConfig {
    pub fn get_model_by_index(&self, index: usize) -> Option<&String> {
        self.llms.get(index).map(|llm| &llm.model)
    }

    pub fn get_api_base_by_index(&self, index: usize) -> Option<&String> {
        self.llms.get(index).map(|llm| &llm.api_base)
    }

    pub fn get_api_key_by_index(&self, index: usize) -> Option<&String> {
        self.llms.get(index).map(|llm| &llm.api_key)
    }

    pub fn get_model_index_by_model(&self, model: &str) -> Option<usize> {
        self.llms.iter().position(|llm| {
            println!("llm.model: {:?} model: {:?}", llm.model, model);
            llm.model.trim() == model.trim()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_by_index() {
        let config = RouterConfig {
            policy: Policy {
                url: "http://0.0.0.0:8080/classify".to_string(),
            },
            llms: vec![
                Llm {
                    name: "llama3".to_string(),
                    api_base: "http://0.0.0.0:8000".to_string(),
                    api_key: "".to_string(),
                    model: "meta/llama3-8b-instruct".to_string(),
                },
                Llm {
                    name: "mistral".to_string(),
                    api_base: "http://0.0.0.0:8001".to_string(),
                    api_key: "".to_string(),
                    model: "mistralai/mistral-7b-instruct-v0.3".to_string(),
                },
                Llm {
                    name: "openai".to_string(),
                    api_base: "https://api.openai.com".to_string(),
                    api_key: "".to_string(),
                    model: "gpt-4o".to_string(),
                },
                Llm {
                    name: "ngc".to_string(),
                    api_base: "https://integrate.api.nvidia.com".to_string(),
                    api_key: "".to_string(),
                    model: "meta/llama-3.1-405b-instruct".to_string(),
                },
            ],
        };

        assert_eq!(
            config.get_model_by_index(0),
            Some(&"meta/llama3-8b-instruct".to_string())
        );
        assert_eq!(
            config.get_model_by_index(1),
            Some(&"mistralai/mistral-7b-instruct-v0.3".to_string())
        );
        assert_eq!(config.get_model_by_index(2), Some(&"gpt-4o".to_string()));
        assert_eq!(
            config.get_model_by_index(3),
            Some(&"meta/llama-3.1-405b-instruct".to_string())
        );
        assert_eq!(config.get_model_by_index(4), None);
    }

    #[test]
    fn test_get_api_base_by_index() {
        let config = RouterConfig {
            policy: Policy {
                url: "http://0.0.0.0:8080/classify".to_string(),
            },
            llms: vec![
                Llm {
                    name: "llama3".to_string(),
                    api_base: "http://0.0.0.0:8000".to_string(),
                    api_key: "".to_string(),
                    model: "meta/llama3-8b-instruct".to_string(),
                },
                Llm {
                    name: "mistral".to_string(),
                    api_base: "http://0.0.0.0:8001".to_string(),
                    api_key: "".to_string(),
                    model: "mistralai/mistral-7b-instruct-v0.3".to_string(),
                },
                Llm {
                    name: "openai".to_string(),
                    api_base: "https://api.openai.com".to_string(),
                    api_key: "".to_string(),
                    model: "gpt-4o".to_string(),
                },
                Llm {
                    name: "ngc".to_string(),
                    api_base: "https://integrate.api.nvidia.com".to_string(),
                    api_key: "".to_string(),
                    model: "meta/llama-3.1-405b-instruct".to_string(),
                },
            ],
        };

        assert_eq!(
            config.get_api_base_by_index(0),
            Some(&"http://0.0.0.0:8000".to_string())
        );
        assert_eq!(
            config.get_api_base_by_index(1),
            Some(&"http://0.0.0.0:8001".to_string())
        );
        assert_eq!(
            config.get_api_base_by_index(2),
            Some(&"https://api.openai.com".to_string())
        );
        assert_eq!(
            config.get_api_base_by_index(3),
            Some(&"https://integrate.api.nvidia.com".to_string())
        );
        assert_eq!(config.get_api_base_by_index(4), None);
    }

    #[test]
    fn test_get_api_key_by_index() {
        let config = RouterConfig {
            policy: Policy {
                url: "http://0.0.0.0:8080/classify".to_string(),
            },
            llms: vec![
                Llm {
                    name: "llama3".to_string(),
                    api_base: "http://0.0.0.0:8000".to_string(),
                    api_key: "".to_string(),
                    model: "meta/llama3-8b-instruct".to_string(),
                },
                Llm {
                    name: "mistral".to_string(),
                    api_base: "http://0.0.0.0:8001".to_string(),
                    api_key: "".to_string(),
                    model: "mistralai/mistral-7b-instruct-v0.3".to_string(),
                },
                Llm {
                    name: "openai".to_string(),
                    api_base: "https://api.openai.com".to_string(),
                    api_key: "sk-proj-someapikey".to_string(),
                    model: "gpt-4o".to_string(),
                },
                Llm {
                    name: "ngc".to_string(),
                    api_base: "https://integrate.api.nvidia.com".to_string(),
                    api_key: "nvapi-someapikey".to_string(),
                    model: "meta/llama-3.1-405b-instruct".to_string(),
                },
            ],
        };

        assert_eq!(config.get_api_key_by_index(0), Some(&"".to_string()));
        assert_eq!(config.get_api_key_by_index(1), Some(&"".to_string()));
        assert_eq!(
            config.get_api_key_by_index(2),
            Some(&"sk-proj-someapikey".to_string())
        );
        assert_eq!(
            config.get_api_key_by_index(3),
            Some(&"nvapi-someapikey".to_string())
        );
        assert_eq!(config.get_api_key_by_index(4), None);
    }

    #[test]
    fn test_get_model_index_by_model() {
        let config = RouterConfig {
            policy: Policy {
                url: "http://0.0.0.0:8080/classify".to_string(),
            },
            llms: vec![
                Llm {
                    name: "llama3".to_string(),
                    api_base: "http://0.0.0.0:8000".to_string(),
                    api_key: "".to_string(),
                    model: "meta/llama3-8b-instruct".to_string(),
                },
                Llm {
                    name: "mistral".to_string(),
                    api_base: "http://0.0.0.0:8001".to_string(),
                    api_key: "".to_string(),
                    model: "mistralai/mistral-7b-instruct-v0.3".to_string(),
                },
                Llm {
                    name: "openai".to_string(),
                    api_base: "https://api.openai.com".to_string(),
                    api_key: "sk-proj-someapikey".to_string(),
                    model: "gpt-4o".to_string(),
                },
                Llm {
                    name: "ngc".to_string(),
                    api_base: "https://integrate.api.nvidia.com".to_string(),
                    api_key: "nvapi-someapikey".to_string(),
                    model: "meta/llama-3.1-405b-instruct".to_string(),
                },
            ],
        };

        assert_eq!(
            config.get_model_index_by_model("meta/llama3-8b-instruct"),
            Some(0)
        );

        assert_eq!(config.get_model_index_by_model("should not exist"), None);
    }
}
