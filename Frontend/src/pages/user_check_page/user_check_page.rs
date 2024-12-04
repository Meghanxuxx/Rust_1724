use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;

#[function_component(UserCheckPage)]
pub fn user_check_page() -> Html {
    let current_time = Local::now().format("%I:%M %p").to_string();
    let user_input = "Input of user"; // 随便放了点东西

    html! {
        <>
            <Header show_line={false} />
            <div class="user-check-page">
                <div class="app-content">
                    <Sidebar />
                    <div class="content-wrapper">
                        <div class="chat-container">
                            <div class="chat-message user-message">
                                <div class="message-header">
                                    <div class="user-avatar">
                                        <div class="avatar-placeholder"></div>
                                    </div>
                                    <span class="user-name">{ "You" }</span>
                                    <span class="timestamp">{ current_time }</span>
                                </div>
                                <div class="message-bubble user-bubble">
                                    <p class="message-text">
                                        { user_input }
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
