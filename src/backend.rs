use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, fs};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}
#[server(endpoint = "call_openai")]
pub async fn call_openai(user_input: String) -> Result<(String), ServerFnError> {
    let openai_file = fs::read_to_string("env/OPENAI_API_KEY").unwrap_or_default();
    let OPENAI_API_KEY: String = env::var("OPENAI_API_KEY").unwrap_or_else(|_| openai_file);

    let client = Client::new();
    let request_body = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            ChatMessage {
                role: "user".to_string(),
                content: user_input,
            },
            ChatMessage {
                role: "system".to_string(),
                content: "Generate the WebGL shader the user wants. Generate only code. Skip explanations.".to_string(),
            },
        ],
    };

    let response = client
        .post(OPENAI_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", OPENAI_API_KEY))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    let json: OpenAIResponse = response.json().await.map_err(|e| e.to_string()).unwrap();
    let response = json.choices.get(0).unwrap().message.content.clone();

    Ok(response)
}

