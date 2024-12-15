use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
use dotenv::dotenv;
use std::env;
use crate::config::AppConfig;

pub async fn get_gpt_response(prompt: &str, config: &AppConfig) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file if present

    // Retrieve API key
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found in .env file");

    // Set up headers for the HTTP request
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Read system prompt from the config
    let system_prompt = config.get_system_prompt();

    // Create HTTP client
    let client = reqwest::Client::new();

    println!("Using system prompt: {}", system_prompt);
    // Prepare the request body
    let body = json!({
        "model": config.openai.model,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": prompt }
        ],
        "max_tokens": config.openai.max_tokens
    });

    // Make the API request
    let response = client
        .post(&config.openai.api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    // Parse the response JSON
    let response_json: serde_json::Value = response.json().await?;
    eprintln!("Raw response JSON: {:?}", response_json);
    if let Some(reply) = response_json["choices"][0]["message"]["content"].as_str() {
        Ok(reply.to_string())
    } else {
        eprintln!("Unexpected response format: {:?}", response_json);
        Err("Unexpected response format".into())
    }
}
