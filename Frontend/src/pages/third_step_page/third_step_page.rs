use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures;
use gloo_net::http::Request;
use crate::types::CoverLetterInput;
use crate::Route;
use yew_router::prelude::*;

fn get_user_id() -> Option<String> {
    if let Some(window) = web_sys::window() {
        // get localStorage
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(uid)) = storage.get_item("user_id") {
                return Some(uid);
            }
        }
    }
    None
}

// 和第一，二步一样的逻辑
async fn submit_third_step(content: &str) -> Result<bool, String> {
    let url = "http://127.0.0.1:8081/api/step3";

    let input_data = CoverLetterInput {
        step: 3,
        content: content.to_string(),
        user_id: get_user_id(),
    };

    let req = Request::post(url)
        .json(&input_data)
        .map_err(|_| "Failed to create request (step3)".to_string())?
        .send()
        .await
        .map_err(|_| "Failed to send request (step3)".to_string())?;

    if req.ok() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[function_component(MessageInput)]
fn message_input() -> Html {
    let input_ref = use_node_ref();
    let navigator = use_navigator().expect("No navigator available for third step");

    let on_submit = {
        let input_ref = input_ref.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let content = input.value();

            if !content.trim().is_empty() {
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match submit_third_step(&content).await {
                        Ok(true) => {
                            navigator.push(&Route::FinalStep);
                        },
                        _ => {
                            web_sys::window().unwrap().alert_with_message("Need more information").ok();
                        }
                    }
                });
            }
            input.set_value("");
        })
    };

    html! {
        <div class="message-input-container">
            <div class="message-input-wrapper">
                <input
                    type="text"
                    class="message-input"
                    placeholder="Any final touches or style preferences?"
                    ref={input_ref}
                />
                <button class="send-button" onclick={on_submit} />
            </div>
        </div>
    }
}

#[function_component(ThirdStepPage)]
pub fn third_step_page() -> Html {
    let current_time = Local::now().format("%I:%M %p").to_string();

    html! {
        <>
            <Header show_line={false} />
            <div class="third-step-page">
                <div class="app-content">
                    <Sidebar />
                    <div class="content-wrapper">
                        <div class="first-step-container">
                            <h1 class="title">{ "Final Step, Let's Personalize the Details" }</h1>
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
                                            { "Do you have any specific tone, style, or length preferences for your cover letter?" }
                                        </p>
                                    </div>
                                </div>
                            </div>
                            <div class="info-cards-container">
                                {
                                    vec![
                                        ("Tone & Style Preferences", "E.g: Formal, friendly, confident..."),
                                        ("Word Count or Length", "E.g: 200 words, 400 words, flexible..."),
                                        ("Target or Focus", "What should we emphasize most?"),
                                        ("Format & Structure", "E.g: Clear sections, concise paragraphs.."),
                                        ("Additional Requests", "Anything extra you'd like us to include?")
                                    ].into_iter().enumerate().map(|(index, (title, subtitle))| {
                                        html! {
                                            <div class={format!("info-card fade-in-card delay-{}", index)}>
                                                <img src="assets/idea.png" alt="Idea Icon" class="info-icon" />
                                                <div class="card-content">
                                                    <h3 class="card-title">{ title }</h3>
                                                    <p class="card-subtitle">{ subtitle }</p>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <MessageInput />
        </>
    }
}