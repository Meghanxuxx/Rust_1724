use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;
use yew_router::prelude::*;
use crate::Route;
use web_sys::AnimationEvent;

#[function_component(FinalStepPage)]
pub fn final_step_page() -> Html {
    let navigator = use_navigator().unwrap();
    let message_complete = use_state(|| true);
    let current_time = Local::now().format("%I:%M %p").to_string();
    
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
                                        { "xxxx" } // 这里放生成的Cover Letter内容
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