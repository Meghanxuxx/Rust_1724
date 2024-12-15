use actix_web::{post, get, web, HttpResponse};
use serde_json::json;
use crate::types::CoverLetterInput;
use crate::AppState;
use crate::openai::client::get_gpt_response;
use crate::config::AppConfig;

pub async fn process_all_steps(user_id: &str, data: Vec<String>, config: &AppConfig) -> HttpResponse {
    let whole_content = data.join("\n");
    //println!("User {}: Received all information! Combined content: {}", user_id, whole_content);

    match get_gpt_response(&whole_content, config).await {
        Ok(response) => HttpResponse::Ok().json(json!({ "role": "GPT response", "content": response })),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "message": "Failed to get GPT response", "error": e.to_string() })),
    }
}

async fn handle_step(
    step: u8,
    input: Option<web::Json<CoverLetterInput>>, // Optional to handle both POST and GET
    app_data: web::Data<AppState>,
) -> HttpResponse {

    let user_id = match &input {
        Some(input_data) => match &input_data.user_id {
            Some(id) if !id.is_empty() => id.clone(),
            _ => {
                return HttpResponse::BadRequest().json(json!({
                    "message": "User ID is required"
                }));
            }
        },
        None => "12345".to_string(), // Default user ID for step 4 (GET)
    };

    let mut user_steps = app_data.user_steps.lock().unwrap();
    let steps = user_steps.entry(user_id.clone()).or_insert_with(|| vec![String::new(); 3]);

    if step == 4 {
        // Final step: Combine all data and process
        return process_all_steps(&user_id, steps.clone(), &app_data.config).await;
    } else if (step as usize) <= steps.len() {
        if let Some(input_data) = input {
            steps[(step - 1) as usize] = input_data.content.clone();
            return HttpResponse::Ok().json(json!({
                "message": format!("Step {} received successfully", step),
                "content": input_data.content
            }));
        }
    }

    HttpResponse::BadRequest().json(json!({
        "message": format!("Invalid step number: {}", step)
    }))
}

#[post("/api/step1")]
pub async fn handle_step_one(input: web::Json<CoverLetterInput>, app_data: web::Data<AppState>) -> HttpResponse {
    handle_step(1, Some(input), app_data).await
}

#[post("/api/step2")]
pub async fn handle_step_two(input: web::Json<CoverLetterInput>, app_data: web::Data<AppState>) -> HttpResponse {
    handle_step(2, Some(input), app_data).await
}

#[post("/api/step3")]
pub async fn handle_step_three(input: web::Json<CoverLetterInput>, app_data: web::Data<AppState>) -> HttpResponse {
    handle_step(3, Some(input), app_data).await
}

#[post("/api/final-step")]
pub async fn handle_final_step(input: web::Json<CoverLetterInput>, app_data: web::Data<AppState>) -> HttpResponse {
    handle_step(4, Some(input), app_data).await
}
