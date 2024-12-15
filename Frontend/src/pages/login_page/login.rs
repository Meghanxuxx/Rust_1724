use crate::Route;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::types::User;
use gloo_net::http::Request;

async fn login(name: &str, pwd: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Please enter username".to_string());
    }
    if pwd.trim().is_empty() {
        return Err("Please enter password".to_string());
    }
    
    let user = User {
        id: String::new(),
        name: name.to_string(),
        password: pwd.to_string(),
        age: None,
        gender: None,
    };

    let request = Request::post("http://127.0.0.1:8081/login");
    let request = match request.json(&user) {
        Ok(request) => request,
        Err(_) => return Err("Data format error".to_string())
    };

    let result = match request.send().await {
        Ok(result) => result,
        Err(_) => return Err("Server connection failed".to_string())
    };

    if result.ok() {
        Ok(())
    } else {
        Err("Wrong username or password".to_string())
    }
}

async fn save_user_to_storage(username: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            storage.set_item("user_name", username).ok();
            storage.set_item("user_id", username).ok();
        }
    }
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let username = use_node_ref();
    let password = use_node_ref();
    let error_message = use_state(|| None::<String>);
    
    let navigator = use_navigator().unwrap();
    let show_password = use_state(|| false);

    let toggle_password = {
        let show_password = show_password.clone();
        Callback::from(move |_| {
            show_password.set(!*show_password);
        })
    };

    let handle_submit = {
        let username = username.clone();
        let password = password.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let name = username.cast::<HtmlInputElement>().unwrap().value();
            let pass = password.cast::<HtmlInputElement>().unwrap().value();

            let error_message = error_message.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match login(&name, &pass).await {
                    Ok(_) => {
                        save_user_to_storage(&name).await;
                        navigator.push(&Route::FirstStep);
                    }
                    Err(msg) => {
                        error_message.set(Some(msg));
                    }
                }
            });
        })
    };

    html! {
        <div class="account-page">
            <div class="account-left">
                <div class="account-quote">
                    <div class="account-quote-mark">{"❝"}</div>
                    <h1 class="account-quote-text">
                        {"Your Career, One"}
                        <br/>
                        {"Letter Closer."}
                    </h1>
                </div>
            </div>
            
            <div class="account-right">
                <Link<Route> to={Route::Home} classes="account-logo">
                    <img src="assets/logo.png" alt="Logo" class="account-logo-img" />
                    <span class="account-logo-text">{"CoverDraft"}</span>
                </Link<Route>>
                
                <div class="account-form">
                    <h2 class="account-title">{"Welcome Back!"}</h2>
                    <p class="account-desc">{"Login to your account"}</p>
                    
                    if let Some(msg) = (*error_message).clone() {
                        <div class="account-error">
                            {msg}
                        </div>
                    }
                    
                    <form onsubmit={handle_submit}>
                        <div class="account-input">
                            <label for="username">{"Username"}</label>
                            <input 
                                type="text"
                                id="username"
                                placeholder="username"
                                ref={username}
                            />
                        </div>
                    
                        <div class="account-input">
                            <label for="password">{"Password"}</label>
                            <div class="account-password">
                                <input 
                                    type={if *show_password { "text" } else { "password" }}
                                    id="password"
                                    placeholder="password"
                                    ref={password}
                                />
                                <button 
                                    type="button"
                                    class="account-password-toggle"
                                    onclick={toggle_password}
                                >
                                    if *show_password {
                                        <img src="assets/eye-off.svg" alt="Hide" />
                                    } else {
                                        <img src="assets/eye.svg" alt="Show" />
                                    }
                                </button>
                            </div>
                        </div>
                        
                        <button type="submit" class="account-submit">
                            {"Login"}
                        </button>
                    </form>
                    
                    <div class="account-login">
                        <Link<Route> to={Route::CreateAccount}>
                            {"Don't have an account? Register here"}
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}