
use yew::prelude::*;
use web_sys::{window, Document, Element};
use serde_json::Value;
use web_sys::HtmlInputElement;
// crate::pages::final_step_page::final_step_page::HistoryItemPage;
// crate::pages::final_step_page::final_step_page::HistoryItem;
// use crate::Header;
use crate::components::{Sidebar, Header};

// #[derive(Deserialize)]

struct HistoryItem {
    id: usize,
    content: String,
}

fn get_history_text() -> Result<String, String> {
    let window = window().ok_or("Failed to get window".to_string())?;
    let storage = window
        .local_storage()
        .map_err(|err| format!("Failed to access local storage: {:?}", err))?
        .ok_or("Local storage is not available".to_string())?;
    
    let history_text = storage
        .get_item("history")
        .map_err(|err| format!("Failed to retrieve history from local storage: {:?}", err))?
        .ok_or("No history found in local storage".to_string())?;

    // parse the history result and query
    
    Ok(history_text)
}

#[function_component(HistoryItemPage)]
pub fn history_page() -> Html {
    let history = get_history_text();

    // html! {
    //     <div class="page-container">
    //         <Header show_line={false} />
    //         // <p>{}</p>
    //         <div class="app-content">
    //             <Sidebar />

    //             <div class="history-page-header">
    //                 <div class="content-wrapper">

    //                 <div class="avatar">
    //                     <img src="assets/avator.png" alt="CoverCraft" class="avatar-image" />
    //                 </div>
    //                     <h1 class = "title">{ "History Page" }</h1>

    //                     <div class="history-page">
    //                     <p>{ history.unwrap_or("No history".to_string()) }</p>
    //                     </div>
    //                 </div>
    //             </div>

    //             // <div class="history-page">
    //             //     <p>{ history.unwrap_or("No history".to_string()) }</p>
    //             // </div>
    //         </div>
    //     </div>
    // }
   

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
                                    // <div class="avatar">
                                        // <img src="assets/avator.png" alt="CoverCraft" class="avatar-image" />
                                    // </div>
                                    <span class="bot-name">{ "CoverCraft" }</span>
                                </div>
                                <div class="message-bubble">
                                <p>{ history.unwrap_or("No history".to_string()) }</p>
                                </div>
                            </div>
                        </div>

                    </div>
                </div>
            </div>
        </div>
    }
    

}