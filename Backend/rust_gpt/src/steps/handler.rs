use actix_web::{post, web, HttpResponse};
use serde_json::json;
use crate::types::CoverLetterInput;
use crate::AppState;
use crate::openai::client::get_gpt_response;
use crate::config::AppConfig;

pub async fn process_all_steps(user_id: &str, data: Vec<String>, config: &AppConfig) -> HttpResponse {
    let whole_content = data.join(" ");
    println!("User {}: Received all information! Combined content: {}", user_id, whole_content);

    match get_gpt_response(&whole_content, config).await {
        Ok(response) => HttpResponse::Ok().json(json!({ "role": "GPT response", "content": response })),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "message": "Failed to get GPT response", "error": e.to_string() })),
    }
}

async fn handle_step(
    step: u8,
    input: web::Json<CoverLetterInput>,
    app_data: web::Data<AppState>,
) -> HttpResponse {
    let user_id = match &input.user_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => {
            return HttpResponse::BadRequest().json(json!({
                "message": "User ID is required"
            }));
        }
    };

    let mut user_steps = app_data.user_steps.lock().unwrap();

    let steps = user_steps.entry(user_id.clone()).or_insert_with(|| vec![String::new(); 3]);
    if step as usize > steps.len() {
        return HttpResponse::BadRequest().json(json!({
            "message": format!("Invalid step number: {}", step)
        }));
    }
    steps[(step - 1) as usize] = input.content.clone();

    if steps.iter().all(|content| !content.is_empty()) {
        let combined_steps = steps.clone();
        user_steps.remove(&user_id);
        return process_all_steps(&user_id, combined_steps, &app_data.config).await;
    }

    HttpResponse::Ok().json(json!({
        "message": format!("Step {} received successfully", step),
        "content": input.content
    }))
}

#[post("/api/step1")]
pub async fn handle_step_one(input: web::Json<CoverLetterInput>, app_data: web::Data<AppState>) -> HttpResponse {
    println!("!!!!!!!!!!!!!!!!!!");
    handle_step(1, input, app_data).await
}

#[post("/api/step2")]
pub async fn handle_step_two(input: web::Json<CoverLetterInput>, app_data: web::Data<AppState>) -> HttpResponse {
    handle_step(2, input, app_data).await
}

#[post("/api/step3")]
pub async fn handle_step_three(input: web::Json<CoverLetterInput>, app_data: web::Data<AppState>) -> HttpResponse {
    handle_step(3, input, app_data).await
}
