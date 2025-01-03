use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub show_line: bool,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
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
                    <li><Link<Route> to={Route::Login} classes="nav-link">{"Sign In"}</Link<Route>></li>
                    <li><Link<Route> to={Route::AboutUs} classes="nav-link">{"About Us"}</Link<Route>></li>
                    <li>
                        <Link<Route> to={Route::CreateAccount} classes="btn-create-account">
                            {"Create account"}
                        </Link<Route>>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
