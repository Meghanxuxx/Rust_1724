use crate::Route;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::types::User;
use gloo_net::http::Request;

// try to login
async fn try_login(name: &str, pwd: &str) -> Result<(), String> {

    if name.trim().is_empty() {
        return Err("Username is empty".to_string());
    }
    if pwd.trim().is_empty() {
        return Err("Password is empty".to_string());
    }
    let user = User {
        id: String::new(),
        name: name.to_string(),
        password: pwd.to_string(),
        age: None,
        gender: None,
    };

    let req = Request::post("http://127.0.0.1:8081/login");
    
    // try to make json
    let req = match req.json(&user) {
        Ok(req) => req,
        Err(_) => return Err("Something broke".to_string())
    };

    // try to send
    let resp = match req.send().await {
        Ok(r) => r,
        Err(_) => return Err("Can't connect to server".to_string())
    };

    if resp.ok() {
        Ok(())
    } else {
        Err("Wrong username or password".to_string())
    }
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    // refs for our inputs
    let name_ref = use_node_ref();
    let pwd_ref = use_node_ref();
    let error = use_state(|| None::<String>);
    
    let nav = use_navigator().unwrap();
    let show_pwd = use_state(|| false);

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
        let error = error.clone();
        let nav = nav.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // get input values
            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let pwd = pwd_ref.cast::<HtmlInputElement>().unwrap().value();

            let error = error.clone();
            let nav = nav.clone();

            // try to login
            wasm_bindgen_futures::spawn_local(async move {
                match try_login(&name, &pwd).await {
                    Ok(_) => {
                        nav.push(&Route::Home);  // go home if login works
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
            });
        })
    };

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
                    <h2 class="account-form-title">{"Welcome Back!"}</h2>
                    <p class="account-form-subtitle">{"Login to your account"}</p>
                    if let Some(err) = (*error).clone() {
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
                                placeholder="username"
                                ref={name_ref}
                            />
                        </div>
                    
                        <div class="account-form-group">
                            <label for="password">{"Password"}</label>
                            <div class="password-input-container">
                                <input 
                                    type={if *show_pwd { "text" } else { "password" }}
                                    id="password"
                                    placeholder="password"
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
                        
                        <button type="submit" class="login-button">
                            {"Login"}
                        </button>
                    </form>
                    
                    <div class="account-login-link">
                        <Link<Route> to={Route::CreateAccount}>
                            {"Don't have an account? Register here"}
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}