use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;

#[function_component(History)]
pub fn history() -> Html {
    // 假装放了点history上来当例子
    let chat_history = vec![
    ("The latest cover letter history", 1),
    // ("COMPsci SICP tutorial course", 2),
    // ("Proxy failure troubleshooting", 3),
    // ("Wake me up when September ends chord", 4),
    // ("Best OASIS songs top 100 all time", 5),
    // ("Fix SSL/TLS Error", 6),
    // ("React component quick fix", 7),
    // ("Next.js 18 documentation", 8),
];


    let is_open = use_state(|| true);
    let on_click = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class={classes!("section", "history", if *is_open { "open" } else { "closed" })}>
            { if *is_open {
                html! { <div class="divider"></div> }
            } else {
                html! {}
            }}
            <div class="section-header" onclick={on_click}>
                <div class="icon-wrap">
                    <img src="assets/history.png" alt="History Icon" class="section-icon" />
                </div>
                <span class="section-title">{ "History" }</span>
                <img src={if *is_open { "assets/up.png" } else { "assets/down.png" }} alt="Toggle Arrow" class="section-arrow" />
            </div>
            { if *is_open {
                html! {
                    <div class="section-content">
                        <div class="history-content">
                            <ul class="history-list">
                                { for chat_history.iter().map(|(name, id)| html! {
                                    <li class="chat_history" key={id.to_string()}>
                                        <Link<Route> to={Route::HistoryItem { id: *id }} classes="history-link">
                                            { name }
                                        </Link<Route>>
                                    </li>
                                }) }
                            </ul>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}