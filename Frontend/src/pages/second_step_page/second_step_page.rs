use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;

#[function_component(MessageInput)]
fn message_input() -> Html {
    html! {
        <div class="message-input-container">
            <div class="message-input-wrapper">
                <input 
                    type="text"
                    class="message-input"
                    placeholder="Message CoverDraft..."
                />
                <button class="send-button" />
            </div>
        </div>
    }
}

#[function_component(SecondStepPage)]
pub fn second_step_page() -> Html {
    let current_time = Local::now().format("%I:%M %p").to_string();

    html! {
        <>
            <Header show_line={false} />
            <div class="second-step-page">
                <div class="app-content">
                    <Sidebar />
                    <div class="content-wrapper">
                        <div class="first-step-container">
                            <h1 class="title">{ "Perfect! Now we'd love to hear about the work experience that makes you stand out" }</h1>
                            <div class="chat-container fade-in">
                                <div class="chat-message">
                                    <div class="message-header">
                                        <div class="avatar">
                                            <img src="assets/avator.png" alt="CoverCraft" class="avatar-image" />
                                        </div>
                                        <span class="bot-name">{ "CoverCraft" }</span>
                                        <span class="timestamp">{ current_time }</span>
                                    </div>
                                    <div class="message-bubble">
                                        <p class="message-text">
                                            { "Your career journey is unique. Share the highlights, and we'll make them shine on paper." }
                                        </p>
                                    </div>
                                </div>
                            </div>
                            <div class="info-cards-container">
                                {
                                    vec![
                                        ("Job Title / Position", "What was your job title?"),
                                        ("Company Name", "Which company did you work for?"),
                                        ("Dates of Employment", "How long did you work there?"),
                                        ("Key Responsibilities", "What were your main responsibilities?"),
                                        ("Achievements / Impact", "What did you accomplish or improve?"),
                                        ("Skills Gained / Developed", "What skills did you develop in this role?")
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
