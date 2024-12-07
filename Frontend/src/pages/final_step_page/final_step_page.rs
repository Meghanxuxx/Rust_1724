
use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;

#[function_component(FinalStepPage)]
pub fn final_step_page() -> Html {
    let current_time = Local::now().format("%I:%M %p").to_string();

    html! {
        <div class="page-container">
            <Header show_line={false} />
            <div class="final-step-page">
                <div class="app-content">
                    <Sidebar />
                    <div class="main-content">
                        <div class="content-wrapper">
                            <div class="final-step-container">
                                <h1 class="title">{ "Your Cover Letter Is Ready!" }</h1>
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
                                                { "xxxx" }  // 这里放生成的Cover Letter内容
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}