use yew::prelude::*;
use crate::components::{Sidebar, Header};
use crate::types::HistoryItem;
use chrono::Local;
use web_sys;

#[function_component(HistoryItemPage)]
pub fn history_page() -> Html {
    let history = use_state(|| Vec::<HistoryItem>::new());
    let time = Local::now().format("%I:%M %p").to_string();
    
    {
        let history = history.clone();
        use_effect_with((), move |_| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(data)) = storage.get_item("chat_history") {
                        if let Ok(items) = serde_json::from_str::<Vec<HistoryItem>>(&data) {
                            history.set(items);
                        }
                    }
                }
            }
            || ()
        });
    }

    html! {
        <div class="page">
            <Header show_line={false} />
            <div class="app-content">
                <Sidebar />
                <div class="content-wrapper">
                    <div class="first-step-container">
                        <h1 class="title">{"Your Cover Letter History"}</h1>
                        
                        if history.is_empty() {
                            <div class="chat-container fade-in">
                                <div class="chat-message">
                                    <div class="message-header">
                                        <div class="avatar">
                                            <img src="/assets/avator.png" alt="CoverCraft" class="avatar-image" />
                                        </div>
                                        <span class="bot-name">{ "CoverCraft" }</span>
                                        <span class="timestamp">{ time.clone() }</span>
                                    </div>
                                    <div class="message-bubble">
                                        <p class="message-text">
                                            { "No history available yet. Generate your first cover letter!" }
                                        </p>
                                    </div>
                                </div>
                            </div>
                        } else {
                            {
                                for history.iter().map(|item| {
                                    html! {
                                        <div class="chat-container fade-in">
                                            <div class="chat-message">
                                                <div class="message-header">
                                                    <div class="avatar">
                                                        <img src="/assets/avator.png" alt="CoverCraft" class="avatar-image" />
                                                    </div>
                                                    <span class="bot-name">{ "CoverCraft" }</span>
                                                    <span class="timestamp">{ time.clone() }</span>
                                                </div>
                                                <div class="message-bubble">
                                                    <p class="message-text">
                                                        { &item.content }
                                                    </p>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}