use crate::Route;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures;
use gloo_net::http::Request;
use serde_json::json;
use crate::types::User;

// try to register user
async fn try_register(user: &User) -> Result<(), String> {
    let req = Request::post("http://127.0.0.1:8081/register");
    
    let req = match req.json(user) {
        Ok(req) => req,
        Err(_) => return Err("cant make json".to_string())
    };

    let response = match req.send().await {
        Ok(r) => r,
        Err(_) => return Err("server not responding".to_string())
    };

    if response.ok() {
        Ok(())
    } else {
        let text = response.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message") {
                if let Some(go_str) = msg.as_str() {
                    return Err(go_str.to_string());
                }
            }
        }
        Err("something went wrong".to_string())
    }
}

#[function_component(CreateAccountPage)]
pub fn signup_page() -> Html {
    // refs for all inputs
    let name_ref = use_node_ref();
    let pwd_ref = use_node_ref();
    let age_ref = use_node_ref();
    let gender_ref = use_node_ref();
    let error_msg = use_state(|| None::<String>);

    // html stuff
    let nav = use_navigator().unwrap();
    let show_pwd = use_state(|| false);  // for showing/hiding password

    // html stuff - toggle password visibility
    let toggle_pwd = {
        let show_pwd = show_pwd.clone();
        Callback::from(move |_| {
            show_pwd.set(!*show_pwd);
        })
    };

    // handle form submit
    let on_submit = {
        let name_ref = name_ref.clone();
        let pwd_ref = pwd_ref.clone();
        let age_ref = age_ref.clone();
        let gender_ref = gender_ref.clone();
        let error_msg = error_msg.clone();
        let nav = nav.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // get values from inputs
            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let pwd = pwd_ref.cast::<HtmlInputElement>().unwrap().value();

            // basic validation
            if name.trim().is_empty() {
                error_msg.set(Some("Username can't be empty.".to_string()));
                return;
            }
            if pwd.trim().is_empty() {
                error_msg.set(Some("Password can't be empty.".to_string()));
                return;
            }

            let age = age_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .ok();
            
            let gender = gender_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();

            // make new user
            let user = User {
                id: Uuid::new_v4().to_string(),  // random id
                name: name.clone(),
                password: pwd.clone(),
                age,  // optional
                gender: Some(gender),
            };

            let error_msg = error_msg.clone();
            let nav = nav.clone();

            // try to register
            wasm_bindgen_futures::spawn_local(async move {
                match try_register(&user).await {
                    Ok(_) => {
                        nav.push(&Route::Login);  // it worked, go to login
                    }
                    Err(e) => {
                        error_msg.set(Some(e));
                    }
                }
            });
        })
    };

    // the actual html
    html! {
        <div class="account-page-container">
            <div class="account-left-panel">
                <div class="account-quote-container">
                    <div class="account-quote-marks">{"❝"}</div>
                    <h1 class="account-quote-text">
                        {"Your Career, One"}
                        <br/>
                        {"Letter Closer."}
                    </h1>
                </div>
            </div>
            
            <div class="account-right-panel">
                <Link<Route> to={Route::Home} classes="account-page-logo">
                    <img src="assets/logo.png" alt="Logo" class="logo-icon" />
                    <span class="logo-text">{"CoverDraft"}</span>
                </Link<Route>>
                
                <div class="account-form-container">
                    <h2 class="account-form-title">{"Get Started Now"}</h2>
                    <p class="account-form-subtitle">{"Enter your info to create an account"}</p>
                    
                    if let Some(err) = (*error_msg).clone() {
                        <div class="error-message">
                            {err}
                        </div>
                    }
                    <form onsubmit={on_submit}>
                        <div class="account-form-group">
                            <label for="username">{"Username"}</label>
                            <input 
                                type="text"
                                id="username"
                                placeholder="Name"
                                ref={name_ref}
                            />
                        </div>
                        
                        <div class="account-form-group">
                            <label for="password">{"Password"}</label>
                            <div class="password-input-container">
                                <input 
                                    type={if *show_pwd { "text" } else { "password" }}
                                    id="password"
                                    placeholder="Password"
                                    ref={pwd_ref}
                                />
                                <button 
                                    type="button"
                                    class="toggle-password-btn"
                                    onclick={toggle_pwd}
                                >
                                    if *show_pwd {
                                        <img src="assets/eye-off.svg" alt="Hide" />
                                    } else {
                                        <img src="assets/eye.svg" alt="Show" />
                                    }
                                </button>
                            </div>
                        </div>
             
                        <div class="account-form-group">
                            <label for="age">{"Age (Optional)"}</label>
                            <input 
                                type="text"
                                id="age"
                                placeholder="Age"
                                ref={age_ref}
                            />
                        </div>
                        
                        <div class="account-form-group">
                            <label for="gender">{"Gender (Optional)"}</label>
                            <input 
                                type="text"
                                id="gender"
                                placeholder="Gender"
                                ref={gender_ref}
                            />
                        </div>
                        
                        <button type="submit" class="account-signup-button">
                            {"Create Account!"}
                        </button>
                    </form>
                
                    <div class="account-login-link">
                        <Link<Route> to={Route::Login}>
                            {"Already have an account? Login here!"}
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}