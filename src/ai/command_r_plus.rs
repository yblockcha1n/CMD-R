use reqwest::Client;
use serde_json::{json, Value};
use std::env;

pub struct CommandRPlus {
    client: Client,
    api_key: String,
    system_prompt: String,
    chat_history: Vec<Value>,
}

impl CommandRPlus {
    pub fn new() -> Self {
        let api_key = env::var("CO_API_KEY").expect("CO_API_KEY must be set");
        let system_prompt = env::var("SYSTEM_PROMPT")
            .unwrap_or_else(|_| "You are a helpful AI assistant.".to_string());
        
        CommandRPlus {
            client: Client::new(),
            api_key,
            system_prompt,
            chat_history: Vec::new(),
        }
    }

    pub async fn chat(&mut self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.cohere.com/v1/chat";
        
        let mut full_history = vec![json!({
            "role": "SYSTEM",
            "message": self.system_prompt
        })];
        full_history.extend(self.chat_history.clone());
        full_history.push(json!({
            "role": "USER",
            "message": message
        }));

        let body = json!({
            "message": message,
            "chat_history": full_history,
            "connectors": [{"id": "web-search"}]
        });

        log::debug!("Sending request to Command-R-Plus API");
        let response = self.client.post(url)
            .header("Authorization", format!("bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let response_body: Value = response.json().await?;
            log::debug!("Received successful response from Command-R-Plus API");
            let ai_response = response_body["text"].as_str().unwrap_or("No response").to_string();
            
            self.chat_history.push(json!({
                "role": "USER",
                "message": message
            }));
            self.chat_history.push(json!({
                "role": "CHATBOT",
                "message": ai_response
            }));

            Ok(ai_response)
        } else {
            let error_message = format!("API request failed with status: {}", response.status());
            log::error!("{}", error_message);
            Err(error_message.into())
        }
    }

    pub fn reset_history(&mut self) {
        self.chat_history.clear();
        log::info!("Chat history has been reset");
    }
}