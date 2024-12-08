use actix_web::{post, web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde_json::json;
use std::sync::Mutex;

mod db;
mod types;

use db::{User, UserStore};
use types::CoverLetterInput;

// Store user data
struct AppState {
    user_store: Mutex<UserStore>
}

// API endpoints:
// - POST /register: User registration
// - POST /login: User login
// - POST /api/step1: first step

// Register
/// Checks:
/// 1. Username cannot be empty
/// 2. Password cannot be empty
/// 3. Username cannot be duplicate
#[post("/register")]
async fn register(new_user: web::Json<User>, app_data: web::Data<AppState>) -> HttpResponse {
    // check if username or password is empty
    if new_user.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Username is empty"
        }));
    }
    if new_user.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Password is empty"
        }));
    }

    let mut store = app_data.user_store.lock().unwrap();

    // try to add the user
    match store.add_user(new_user.into_inner()) {
        Ok(true) => HttpResponse::Ok().json(json!({
            "message": "Welcome!:D"
        })),
        _ => HttpResponse::BadRequest().json(json!({
            "message": "Username already exists"
        }))
    }
}

// login stuff
/// Checks:
/// 1. Username cannot be empty
/// 2. Password cannot be empty
/// 3. Username and password must match
#[post("/login")]
async fn login(login_data: web::Json<User>, app_data: web::Data<AppState>) -> HttpResponse {
    if login_data.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Username is empty"
        }));
    }
    if login_data.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "message": "Password is empty"
        }));
    }

    let store = app_data.user_store.lock().unwrap();
    
    // check if user exists and password is correct
    match store.verify_login(&login_data.name, &login_data.password) {
        Ok(true) => HttpResponse::Ok().json(json!({
            "message": "Login success"
        })),
        _ => HttpResponse::BadRequest().json(json!({
            "message": "Wrong username or password"
        }))
    }
}

/// Step1
///
/// Input format:
/// ```json
/// {
///     "step": 1,
///     "content": "User input content",
///     "user_id": "User ID (Null if not logged in)"
/// }
/// ```
/// 
/// Current response format:
/// ```json
/// {
///     "message": "Success",
///     "content": "User input content"
/// }
/// ```
#[post("/api/step1")]
async fn handle_step_one(input: web::Json<CoverLetterInput>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "Step 1 received successfully",
        "content": input.content
    }))
}

//step2 and step3 is similar to step1

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init the user store
    let store = UserStore::new().expect("Failed to create user store");
    let app_state = web::Data::new(AppState {
        user_store: Mutex::new(store)
    });

    println!("Server running at http://localhost:8081");

    // start server
    HttpServer::new(move || {
        // CORS config for frontend at http://localhost:8080 or http://127.0.0.1:8080
        // Using CORS because frontend and backend are running in separate terminals
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials()
            .max_age(3600);

        // configure app
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(register)
            .service(login)
            .service(handle_step_one)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
} 