use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub show_line: bool,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let sign_in_onclick = Callback::from(|_| {
        // TODO: 登录功能
    });

    let about_us_onclick = Callback::from(|_| {
        // TODO: AboutUs功能
    });

    let create_account_onclick = Callback::from(|_| {
        // TODO: 创建账户功能
    });

    html! {
        <header class={classes!("header", if props.show_line { "with-line" } else { "no-line" })}>
            <nav class="navbar">
                <Link<Route> to={Route::Home} classes="logo-link">
                    <div class="logo">
                        <img src="assets/logo.png" alt="Logo" class="logo-icon" />
                        <span class="logo-text">{"CoverDraft"}</span>
                    </div>
                </Link<Route>>
                <ul class="nav-links">
                    <li><a href="#signin" class="nav-link" onclick={sign_in_onclick}>{"Sign In"}</a></li>
                    <li><a href="#about" class="nav-link" onclick={about_us_onclick}>{"About Us"}</a></li>
                    <li>
                        <a href="#create-account" class="btn-create-account" onclick={create_account_onclick}>
                            {"Create account"}
                        </a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
