
use yew::prelude::*;
use web_sys::{window, Document, Element};
use serde_json::Value;
use web_sys::HtmlInputElement;
use crate::Header;
// crate::pages::final_step_page::HistoryItem;

// fetch the parsed output from final_step_page

// async store the response and update the sidebar
// modify the sidebar of history list to be a button of showing all the history
// add a new page for showing the history list
// add a new page for showing the history item

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

    Ok(history_text)
}


#[function_component(HistoryItemPage)]
pub fn history_page() -> Html {
    let history = get_history_text();

    html! {
        <div class="page-container">
            <Header show_line={false} />
            <div class = "history-page">
                // <p>{ format!("History page") }</p>
                <p>{ history.unwrap_or("No history".to_string()) }</p>
            </div>
        </div>
    }

    

}
