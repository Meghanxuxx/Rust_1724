mod config;
mod db;
mod account;
mod openai;
mod steps;
mod types;

use actix_web::{http, web, App, HttpServer};
use config::AppConfig;
use std::sync::Mutex;
use std::collections::HashMap;
use db::UserStore;
use account::{login::login, register::register};
use steps::handler::{handle_step_one, handle_step_two, handle_step_three, handle_final_step};
use crate::http::header::CONTENT_TYPE;
use actix_cors::Cors;

// Store user data and per-user step content
struct AppState {
    user_store: Mutex<UserStore>,
    user_steps: Mutex<HashMap<String, Vec<String>>>, // Map user_id to step data
    config: AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::from_file();

    // Print the system prompt for debugging
    let system_prompt = config.get_system_prompt();
    println!("Loaded system prompt: {}", system_prompt);

    // Initialize the user store and user_steps map
    let store = UserStore::new().expect("Failed to create user store");
    let app_state = web::Data::new(AppState {
        user_store: Mutex::new(store),
        user_steps: Mutex::new(HashMap::new()),
        config: config.clone(), // Clone the configuration
    });

    // Clone the config for use outside the closure
    let server_host = app_state.config.server.host.clone();
    let server_port = app_state.config.server.port;

    println!("Server running at http://{}:{}", server_host, server_port);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .wrap(
            Cors::default()
                .allowed_origin("http://127.0.0.1:8080")
                .allowed_origin("http://localhost:8000") // Replace with your front-end origin
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![CONTENT_TYPE])
                .max_age(3600),
            )
            .app_data(app_state.clone())
            .service(register)
            .service(login)
            .service(handle_step_one)
            .service(handle_step_two)
            .service(handle_step_three)
            .service(handle_final_step)
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}

