use actix_web::{post, web, HttpResponse};
use serde_json::json;
use crate::db::User;
use crate::AppState;

#[post("/register")]
pub async fn register(new_user: web::Json<User>, app_data: web::Data<AppState>) -> HttpResponse {
    if new_user.name.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({ "message": "Username is empty" }));
    }
    if new_user.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(json!({ "message": "Password is empty" }));
    }

    let mut store = app_data.user_store.lock().unwrap();

    match store.add_user(new_user.into_inner()) {
        Ok(true) => HttpResponse::Ok().json(json!({ "message": "Welcome!:D" })),
        _ => HttpResponse::BadRequest().json(json!({ "message": "Username already exists" })),
    }
}
