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
                                        <span class="timestamp">{ current_time }</span>
                                    </div>
                                    <div class="message-bubble">
                                        <p class="message-text">
                                            { "Every detail matters. Do you have a preferred tone, word count, or specific style? Let's make this cover letter feel just right for you." }
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
