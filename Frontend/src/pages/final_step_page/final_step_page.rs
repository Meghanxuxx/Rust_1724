use serde_json::error;
use yew::prelude::*;
use crate::components::{Sidebar, Header};
use chrono::Local;
use yew_router::prelude::*;
use crate::Route;
use web_sys::AnimationEvent;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use web_sys::console;

// Fetch the combined response from the backend
async fn fetch_final_response() -> Result<String, gloo_net::Error> {

    let response = Request::get("http://127.0.0.1:8081/api/final-step")
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str::<serde_json::Value>(&response) {
        Ok(parsed) => {
            if let Some(content) = parsed.get("content").and_then(|c| c.as_str()) {
                Ok(content.to_string())
            } else {
                Ok("Invalid response format!".to_string())
            }
        }
        Err(error) => Ok(format!("Failed to parse server response {}", error)),
    }
}

#[function_component(FinalStepPage)]
pub fn final_step_page() -> Html {
    let navigator = use_navigator().unwrap();
    let message_complete = use_state(|| true);
    let current_time = Local::now().format("%I:%M %p").to_string();
    let generated_text = use_state(|| "Loading...".to_string());

    let user_id = Some("12345".to_string());
    // Fetch the backend response asynchronously
    {
        let generated_text_clone = generated_text.clone();
        use_effect_with((), move |_| {
            let generated_text = generated_text_clone.clone();
            spawn_local(async move {
                match fetch_final_response().await {
                    Ok(response) => generated_text.set(response),
                    Err(error) => generated_text.set(format!("Failed to fetch data: {}", error)),
                }
            });
            || ()
        });
    }

    let start_new = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::FirstStep);
        })
    };

    html! {
        <div class="page">
            <Header show_line={false} />
            <div class="app-content">
                <Sidebar />
                <div class="content-wrapper">
                    <div class="final-step-container">
                        <h1 class="title">{"Your Cover Letter is Ready!"}</h1>
                        
                        <div class="chat-container fade-in">
                            <div class="chat-message">
                                <div class="message-header">
                                    <div class="avatar">
                                        <img src="assets/avator.png" alt="CoverCraft" class="avatar-image" />
                                    </div>
                                    <span class="bot-name">{ "CoverCraft" }</span>
                                    <span class="timestamp">{ current_time.clone() }</span>
                                </div>
                                <div class="message-bubble">
                                    <p class="message-text">
                                        { (*generated_text).clone() }
                                    </p>
                                </div>
                            </div>
                        </div>

                        if *message_complete {
                            <div class="buttons-container">
                                <button class="continue-button" onclick={start_new}>
                                    {"Start New Conversation"}
                                </button>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}




// #[function_component(FinalStepPage)]
// pub fn final_step_page() -> Html {
//     let navigator = use_navigator().unwrap();
//     let message_complete = use_state(|| true);
//     let current_time = Local::now().format("%I:%M %p").to_string();
//     let generated_text = use_state(|| "Loading...".to_string()); // State to hold the backend response


//     // Fetch the backend response asynchronously
//     {
//         let generated_text_clone = generated_text.clone();
//         use_effect_with((), move |_| {
//             let generated_text = generated_text_clone.clone();
//             spawn_local(async move {
//                 match fetch_cover_letter().await {
//                     Ok(response) => generated_text.set(response),
//                     Err(_) => generated_text.set("Failed to fetch data".to_string()),
//                 }
//             });
//             || ()
//         });
//     }

//     let start_new = {
//         let navigator = navigator.clone();
//         Callback::from(move |_| {
//             navigator.push(&Route::FirstStep);
//         })
//     };

//     html! {
//         <div class="page">
//             <Header show_line={false} />
//             <div class="app-content">
//                 <Sidebar />
//                 <div class="content-wrapper">
//                     <div class="final-step-container">
//                         <h1 class="title">{"Your Cover Letter is Ready!"}</h1>
                        
//                         <div class="chat-container fade-in">
//                             <div class="chat-message">
//                                 <div class="message-header">
//                                     <div class="avatar">
//                                         <img src="assets/avator.png" alt="CoverCraft" class="avatar-image" />
//                                     </div>
//                                     <span class="bot-name">{ "CoverCraft" }</span>
//                                     <span class="timestamp">{ current_time.clone() }</span>
//                                 </div>
//                                 <div class="message-bubble">
//                                     <p class="message-text">
//                                         // { "xxxx" } // 这里放生成的Cover Letter内容
//                                         { (*generated_text).clone() }
//                                     </p>
//                                 </div>
//                             </div>
//                         </div>

//                         if *message_complete {
//                             <div class="buttons-container">
//                                 <button class="continue-button" onclick={start_new}>
//                                     {"Start New Conversation"}
//                                 </button>
//                             </div>
//                         }
//                     </div>
//                 </div>
//             </div>
//         </div>
//     }
// }

// async fn fetch_cover_letter() -> Result<String, gloo_net::Error> {
//     let response = Request::get("http://127.0.0.1:8080/final-step")
//         .send()
//         .await?
//         .text()
//         .await?;
// //response_json["choices"][0]["message"]["content"].as_str()
//     match serde_json::from_str::<serde_json::Value>(&response) {
//         Ok(parsed) => {
//             if let Some(content) = parsed.get("choices")
//                 .and_then(|choices| choices.get(0))
//                 .and_then(|choice| choice.get("message"))
//                 .and_then(|message| message.get("content"))
//                 .and_then(|content| content.as_str()) {
//                 Ok(content.to_string())
//             } else {
//                 // Debug unexpected structure
//                 eprintln!("Unexpected response structure: {:?}", parsed);
//                 Ok("Invalid response format!".to_string())
//             }
//         }
//         Err(err) => {
//             console::log_1(&format!("Error parsing JSON: {:?}, response: {}", err, response).into());
//             // eprintln!("Error parsing response JSON: {:?}, raw response: {}", err, response);
//             Ok("Invalid response format!!!".to_string())
//         }
//     }
    
// }