use serde::Deserialize; // Ensure serde is imported
use config::File;
use config::FileFormat;
// use std::env;
// use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct OpenAIConfig {
    pub model: String,
    pub max_tokens: u16,
    pub system_prompt_file: String,
    pub api_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)] 
pub struct AppConfig {
    pub openai: OpenAIConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_file() -> Self {
        // let current_dir = env::current_dir().expect("Failed to get current directory");
        // let config_path = current_dir.join("src").join("config.toml");

        let settings = config::Config::builder()
            .add_source(File::new("src/config/config.toml", FileFormat::Toml))
            .build()
            .expect("Failed to load config file");

        settings
            .try_deserialize::<AppConfig>() // Correct method for deserialization
            .expect("Failed to parse config")
    }

    /// Reads the system prompt from the file specified in the OpenAI config
    pub fn get_system_prompt(&self) -> String {
        let file_path = &self.openai.system_prompt_file;
        std::fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("Warning: Failed to read system prompt from file: {}", file_path);
            "You are a helpful assistant.".to_string()
        })
    }
}