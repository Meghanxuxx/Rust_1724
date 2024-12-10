use yew::prelude::*;
use crate::components::sidebar::steps::Steps;
use crate::components::sidebar::history::History;
use yew_router::prelude::Link;
use crate::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let username = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("user_name").ok().flatten())
        .unwrap_or_else(|| String::from("User"));

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
                    <button class="settings-btn">
                        <img src="assets/setting.png" alt="Settings Icon" class="settings-icon" />
                    </button>
                </div>
            </div>
        </aside>
    }
}
