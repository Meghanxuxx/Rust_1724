use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures;
use gloo_net::http::Request;
use crate::types::CoverLetterInput;
use crate::Route;
use yew_router::prelude::*;

// 从localStorage里获取用户ID
// 如果用户登录了就能获取到，没登录就是None
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

/// 把用户输入的内容，step=1和user_id一起POST给 /api/step1 接口。
async fn submit_first_step(content: &str) -> Result<bool, String> {
    // 后端API地址
    let url = "http://127.0.0.1:8081/api/step1";

    // 准备发送的数据
    let input_data = CoverLetterInput {
        step: 1,
        content: content.to_string(),
        user_id: get_user_id(),
    };

    // 发送POST请求
    let req = Request::post(url)
        .json(&input_data)
        .map_err(|_| "Failed to create request".to_string())?
        .send()
        .await
        .map_err(|_| "Failed to send request".to_string())?;

    if req.ok() {
        Ok(true)
    } else {
        // 如果需要可从后端返回的json中获取错误信息提示用户
        Ok(false)
    }
}

/// 用户在输入框里输入自己的信息，然后点击按钮提交给后端
#[function_component(MessageInput)]
fn message_input() -> Html {
    let input_ref = use_node_ref();
    // 导航器，用于页面跳转
    let navigator = use_navigator().expect("No navigator available");
    let on_submit = {
        let input_ref = input_ref.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            // 获取输入的值
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let content = input.value();

            if !content.trim().is_empty() {
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match submit_first_step(&content).await {
                        Ok(true) => {
                            // 成功后跳转到第二步
                            navigator.push(&Route::SecondStep);
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
                    placeholder="Please tell us about yourself..."
                    ref={input_ref}
                />
                <button class="send-button" onclick={on_submit} />
            </div>
        </div>
    }
}

#[function_component(FirstStepPage)]
pub fn first_step_page() -> Html {
    let current_time = Local::now().format("%I:%M %p").to_string();

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
                                            <span class="timestamp">{ current_time.clone() }</span>
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
                                        vec![
                                            ("Education Background", "What is your degree and school?"),
                                            ("Accomplishments & Honors", "Any recognitions you'd want to highlight?"),
                                            ("Skills", "Both soft & technical skills"),
                                            ("Projects", "Any projects that shows your abilities?"),
                                            ("Personal Information", "Ex. Name, Address, Phone Number...")
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
            </div>
            <MessageInput />
        </div>
    }
}
