use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures;
use gloo_net::http::Request;
use crate::types::CoverLetterInput;
use crate::Route;
use yew_router::prelude::*;

// 获取用户ID
fn get_user_id() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item("user_id").ok()?
}

// 提交第三步的内容
async fn send_step_three(content: String) -> Result<(), String> {
    let input = CoverLetterInput {
        step: 3,
        content,
        user_id: get_user_id(),
    };

    let response = Request::post("http://127.0.0.1:8081/api/step3")
        .header("Content-Type", "application/json")
        .json(&input)
        .map_err(|err| format!("Failed to create request: {}", err))?
        .send()
        .await
        .map_err(|err| format!("Failed to send request: {}", err))?;

    // Log the status code and response body
    let text = response.text().await.unwrap_or_else(|_| "Failed to read response text".to_string());

    if response.ok() {
        Ok(())
    } else {
        Err(format!("Server returned an error: {}", text))
    }
}

#[function_component(ChatInput)]
fn chat_input() -> Html {
    let input = use_node_ref();
    let navigator = use_navigator().expect("Navigator not found");
    let on_send = {
        let input = input.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let text = input.cast::<HtmlInputElement>().unwrap().value();
            
            if !text.trim().is_empty() {
                let navigator = navigator.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    match send_step_three(text).await {
                        Ok(_) => navigator.push(&Route::FinalStep),
                        Err(error) => {
                            web_sys::window()
                                .unwrap()
                                .alert_with_message(&error)
                                .ok();
                        }
                    }
                });
            }
            
            input.cast::<HtmlInputElement>().unwrap().set_value("");
        })
    };

    html! {
        <div class="message-input-container">
            <div class="message-input-wrapper">
                <input
                    type="text"
                    class="message-input"
                    placeholder="Tell us your style preferences..."
                    ref={input}
                />
                <button class="send-button" onclick={on_send} />
            </div>
        </div>
    }
}

#[function_component(ThirdStepPage)]
pub fn third_step_page() -> Html {
    let time = Local::now().format("%I:%M %p").to_string();
    let info_cards = vec![
        ("Tone & Style Preferences", "E.g: Formal, friendly, confident..."),
        ("Word Count or Length", "E.g: 200 words, 400 words, flexible..."),
        ("Target or Focus", "What should we emphasize most?"),
        ("Format & Structure", "E.g: Clear sections, concise paragraphs.."),
        ("Additional Requests", "Anything extra you'd like us to include?")
    ];

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
                                        <span class="timestamp">{ time }</span>
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
                                    info_cards.into_iter().enumerate().map(|(index, (title, desc))| {
                                        html! {
                                            <div class={format!("info-card fade-in-card delay-{}", index)}>
                                                <img src="assets/idea.png" alt="Idea" class="info-icon" />
                                                <div class="card-content">
                                                    <h3 class="card-title">{ title }</h3>
                                                    <p class="card-subtitle">{ desc }</p>
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
            <ChatInput />
        </>
    }
}
