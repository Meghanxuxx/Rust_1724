use crate::Route;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures;
use gloo_net::http::Request;
use serde_json::json;
use crate::types::User;

async fn signup(user: &User) -> Result<(), String> {
    let request = Request::post("http://127.0.0.1:8081/register");
    
    let request = match request.json(user) {
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
        let text = result.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message") {
                if let Some(message) = msg.as_str() {
                    return Err(message.to_string());
                }
            }
        }
        Err("Registration Failed".to_string())
    }
}

#[function_component(CreateAccountPage)]
pub fn create_account() -> Html {
    let username = use_node_ref();
    let password = use_node_ref();
    let age = use_node_ref();
    let gender = use_node_ref();
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
        let age = age.clone();
        let gender = gender.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let name = username.cast::<HtmlInputElement>().unwrap().value();
            let pass = password.cast::<HtmlInputElement>().unwrap().value();

            if name.trim().is_empty() {
                error_message.set(Some("Please enter your user name".to_string()));
                return;
            }
            if pass.trim().is_empty() {
                error_message.set(Some("Please enter your password".to_string()));
                return;
            }

            let user_age = age
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .ok();
            
            let user_gender = gender
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            let new_user = User {
                id: Uuid::new_v4().to_string(),
                name,
                password: pass,
                age: user_age,
                gender: Some(user_gender),
            };

            let error_message = error_message.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match signup(&new_user).await {
                    Ok(_) => {
                        navigator.push(&Route::Login);
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
                    <h2 class="account-title">{"Get Started Now"}</h2>
                    <p class="account-desc">{"Enter your info to create an account"}</p>
                    
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
                                placeholder="Name"
                                ref={username}
                            />
                        </div>
                        
                        <div class="account-input">
                            <label for="password">{"Password"}</label>
                            <div class="account-password">
                                <input 
                                    type={if *show_password { "text" } else { "password" }}
                                    id="password"
                                    placeholder="Password"
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
             
                        <div class="account-input">
                            <label for="age">{"Age (Optional)"}</label>
                            <input 
                                type="text"
                                id="age"
                                placeholder="Age"
                                ref={age}
                            />
                        </div>
                        
                        <div class="account-input">
                            <label for="gender">{"Gender (Optional)"}</label>
                            <input 
                                type="text"
                                id="gender"
                                placeholder="Gender"
                                ref={gender}
                            />
                        </div>
                        
                        <button type="submit" class="account-submit">
                            {"Create Account!"}
                        </button>
                    </form>
                
                    <div class="account-login">
                        <Link<Route> to={Route::Login}>
                            {"Already have an account? Login here!"}
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}