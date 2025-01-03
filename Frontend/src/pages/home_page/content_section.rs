use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use sha2::{Sha256, Digest};
use rand::Rng;
use web_sys::window;

fn generate_encrypted_user_id() -> String {
    let random_id: u32 = rand::thread_rng().gen();
    let mut hasher = Sha256::new();
    hasher.update(random_id.to_string());
    format!("{:x}", hasher.finalize())
}

#[function_component(ContentSection)]
pub fn content_section() -> Html {
    let navigator = use_navigator().unwrap();

    let get_cover_letter_onclick = Callback::from(move |_| {
        let user_id = generate_encrypted_user_id();
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            storage.set_item("user_id", &user_id).unwrap();
        }
        navigator.push(&Route::FirstStep);
    });

    html! {
        <section class="content-section">
            <div class="left-panel">
                <div class="contents">
                    <p class="content-title">
                        { "A " }
                        <span class="highlight">{ "Cover Letter" }</span>
                        { " As Unique As You Are" }
                    </p>
                    <p class="content-subtitle">
                        { "Crafting The Perfect First Impression Using AI," }<br/>
                        { "One Letter At A Time" }
                    </p>
                </div>
                <button class="btn-primary" onclick={get_cover_letter_onclick}>
                    <span class="plus-icon">{ "+" }</span>
                    { "Get My Cover Letter" }
                </button>
            </div>
            <div class="right-panel">
                <div class="image-wrapper">
                    <div class="image-group">
                        <img src="assets/ShapeC.png" alt="C-shape" class="c-shape-image" />
                        <img src="assets/sample.png" alt="Sample Document" class="document-img" />
                        <img src="assets/star.png" alt="Star 1" class="star star-1" />
                        <img src="assets/star.png" alt="Star 2" class="star star-2" />
                    </div>
                </div>
            </div>
        </section>
    }
}
