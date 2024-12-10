use yew::prelude::*;
use crate::components::sidebar::steps::Steps;
use crate::components::sidebar::history::History;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let navigator = use_navigator().unwrap();
    let show_modal = use_state(|| false);
    
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let saved_name = storage.get_item("user_name").unwrap_or(None);
    let username = match saved_name {
        Some(name) => name,
        None => String::from("User")
    };

    let logged_in = username != "User";

    let handle_settings = {
        let nav = navigator.clone();
        let modal = show_modal.clone();
        
        Callback::from(move |_| {
            if logged_in {
                modal.set(true);
            } else {
                nav.push(&Route::Login);
            }
        })
    };

    let do_logout = {
        let nav = navigator.clone();
        let modal = show_modal.clone();
        
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            storage.remove_item("user_name").ok();
            
            modal.set(false);
            nav.push(&Route::Login);
        })
    };

    let hide_modal = {
        let modal = show_modal.clone();
        Callback::from(move |_| modal.set(false))
    };

    html! {
        <aside class="sidebar">
            <div class="sidebar-content">
                <Link<Route> to={Route::Home} classes="logo-link">
                    <div class="logo">
                        <img src="assets/logo.png" alt="Logo" class="logo-icon" />
                        <span class="logo-text">{"CoverCraft"}</span>
                    </div>
                </Link<Route>>
                <Steps />
                <div class="scrollable-content">
                    <History />
                </div>
                <div class="sidebar-spacer"></div>

                <div class="more-features">
                    <div class="more-features-card">
                        <div class="features-icon-container">
                            <img src="assets/more.png" alt="Wrench Icon" class="features-icon" />
                        </div>
                        <div class="more-features-title">{"More Features Are Coming Up..."}</div>
                        <div class="more-features-text">{"Still Working on it"}</div>
                    </div>
                </div>

                <div class="user-profile">
                    <div class="user-info">
                        <div class="user-avatar"></div>
                        <span class="user-name">{ username }</span>
                    </div>
                    <button class="settings-btn" onclick={handle_settings}>
                        <img src="assets/setting.png" alt="Settings Icon" class="settings-icon" />
                    </button>
                </div>
            </div>

            if *show_modal && logged_in {
                <div class="modal-backdrop">
                    <div class="modal-content">
                        <h2>{"Confirm Logout"}</h2>
                        <p>{"Are you sure you want to logout?"}</p>
                        <div class="modal-buttons">
                            <button class="modal-button cancel" onclick={hide_modal}>
                                {"Cancel"}
                            </button>
                            <button class="modal-button confirm" onclick={do_logout}>
                                {"Logout"}
                            </button>
                        </div>
                    </div>
                </div>
            }
        </aside>
    }
}
