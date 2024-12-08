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

// 提交第一步的内容
async fn send_step_one(content: String) -> Result<(), String> {
    let input = CoverLetterInput {
        step: 1,
        content,
        user_id: get_user_id(),
    };

    let response = Request::post("http://127.0.0.1:8081/api/step1")
        .json(&input)
        .map_err(|_| "Failed to create request")?
        .send()
        .await
        .map_err(|_| "Server error")?;

    if response.ok() {
        Ok(())
    } else {
        Err("Please provide more information".to_string())
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
                    match send_step_one(text).await {
                        Ok(_) => navigator.push(&Route::SecondStep),
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
                    placeholder="Tell us about yourself..."
                    ref={input}
                />
                <button class="send-button" onclick={on_send} />
            </div>
        </div>
    }
}

#[function_component(FirstStepPage)]
pub fn first_step_page() -> Html {
    let time = Local::now().format("%I:%M %p").to_string();

    let info_cards = vec![
        ("Education Background", "What is your degree and school?"),
        ("Accomplishments & Honors", "Any recognitions you'd want to highlight?"),
        ("Skills", "Both soft & technical skills"),
        ("Projects", "Any projects that shows your abilities?"),
        ("Personal Information", "Ex. Name, Address, Phone Number...")
    ];

    html! {
        <div class="page-container">
            <Header show_line={false} />
            <div class="first-step-page">
                <div class="app-content">
                    <Sidebar />
                    <div class="main-content">
                        <div class="content-wrapper">
                            <div class="first-step-container">
                                <h1 class="title">{ "Tell Us More About You" }</h1>
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
                                                { "Hi! To help us create a cover letter that truly reflects you, let's start with a few details about who you are." }
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
            </div>
            <ChatInput />
        </div>
    }
}
