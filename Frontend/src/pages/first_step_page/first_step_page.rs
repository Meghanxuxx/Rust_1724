use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;

#[derive(Properties, PartialEq)]
pub struct InfoCardProps {
    pub title: String,
    pub subtitle: String,
}

#[function_component(InfoCard)]
fn info_card(props: &InfoCardProps) -> Html {
    html! {
        <div class="info-card">
            <img src="assets/idea.png" alt="Idea Icon" class="info-icon" />
            <div class="card-content">
                <h3 class="card-title">{ &props.title }</h3>
                <p class="card-subtitle">{ &props.subtitle }</p>
            </div>
        </div>
    }
}

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

#[function_component(FirstStepPage)]
pub fn first_step_page() -> Html {
    let current_time = Local::now().format("%I:%M %p").to_string();

    html! {
        <>
            <Header show_line={false} />
            <div class="first-step-page">
                <div class="app-content">
                    <Sidebar />
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
                                        <span class="timestamp">{ current_time }</span>
                                    </div>
                                    <div class="message-bubble">
                                        <p class="message-text">
                                            { "Hi! To help us create a cover letter that truly reflects you, let's start with a few details about who you are, any of the following can works:" }
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
            <MessageInput />
        </>
    }
}
