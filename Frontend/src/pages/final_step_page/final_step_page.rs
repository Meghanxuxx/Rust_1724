use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;
use yew_router::prelude::*;
use crate::Route;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use crate::types::{CoverLetterInput, HistoryItem};
use serde_json::Value;

fn get_user_id() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item("user_id").ok()?
}

async fn save_to_history(content: &str) -> Result<(), String> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let new_item = HistoryItem {
                id: 1,
                content: content.to_string(),
            };
            
            let history_items = vec![new_item];
            
            if let Ok(json) = serde_json::to_string(&history_items) {
                storage.set_item("chat_history", &json).ok();
            }
        }
    }
    Ok(())
}

async fn fetch_final_response() -> Result<String, String> {
    let input = CoverLetterInput {
        step: 4,
        content: "".to_string(),
        user_id: get_user_id(),
    };

    let response = Request::post("http://127.0.0.1:8081/api/final-step")
        .header("Content-Type", "application/json")
        .json(&input)
        .map_err(|err| format!("Failed to create request: {}", err))?
        .send()
        .await
        .map_err(|err| format!("Failed to send request: {}", err))?;

    let response_body = response.text().await.unwrap_or_else(|_| "Failed to read response text".to_string());
    let parsed_json: Value = serde_json::from_str(&response_body)
        .map_err(|err| format!("Failed to parse JSON: {}", err))?;

    let content = parsed_json
        .get("content")
        .ok_or("Failed to extract 'content' field as a string")?;

    let formatted_text = content.to_string().replace("\\n", "\n");
    save_to_history(&formatted_text).await?;
    
    Ok(formatted_text)
}


#[function_component(FinalStepPage)]
pub fn final_step_page() -> Html {
    let navigator = use_navigator().unwrap();
    let message_complete = use_state(|| true);
    let current_time = Local::now().format("%I:%M %p").to_string();
    let generated_text = use_state(|| "Loading...".to_string());
    {
        let generated_text_clone = generated_text.clone();
        use_effect_with((), move |_| {
            let generated_text = generated_text_clone.clone();
            spawn_local(async move {
                match fetch_final_response().await {
                    Ok(response) => generated_text.set(response),
                    Err(error) => generated_text.set(format!("Failed to fetch data: {}", error)),
                }
            });
            || ()
        });
    }

    let start_new = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::FirstStep);
        })
    };

    html! {
        <div class="page">
            <Header show_line={false} />
            <div class="app-content">
                <Sidebar />
                <div class="content-wrapper">
                    <div class="final-step-container">
                        <h1 class="title">{"Your Cover Letter is Ready!"}</h1>
                        
                        <div class="chat-container fade-in">
                            <div class="chat-message">
                                <div class="message-header">
                                    <div class="avatar">
                                        <img src="assets/avator.png" alt="CoverCraft" class="avatar-image" />
                                    </div>
                                    <span class="bot-name">{ "CoverCraft" }</span>
                                    <span class="timestamp">{ current_time.clone() }</span>
                                </div>
                                <div class="message-bubble">
                                    <p class="message-text">
                                        { (*generated_text).clone() }
                                    </p>
                                </div>
                            </div>
                        </div>

                        if *message_complete {
                            <div class="buttons-container">
                                <button class="continue-button" onclick={start_new}>
                                    {"Start New Conversation"}
                                </button>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
