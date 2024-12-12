use actix_web::{post, web, HttpResponse};
use serde_json::json;
use crate::db::User;
use crate::AppState;

#[post("/login")]
pub async fn login(login_data: web::Json<User>, app_data: web::Data<AppState>) -> HttpResponse {
    if login_data.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({ "message": "Username is empty" }));
    }
    if login_data.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({ "message": "Password is empty" }));
    }

    let store = app_data.user_store.lock().unwrap();

    match store.verify_login(&login_data.name, &login_data.password) {
        Ok(true) => HttpResponse::Ok().json(json!({ "message": "Login success" })),
        _ => HttpResponse::BadRequest().json(json!({ "message": "Wrong username or password" })),
    }
}
