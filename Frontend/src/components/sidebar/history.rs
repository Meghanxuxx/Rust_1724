use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;

#[function_component(History)]
pub fn history() -> Html {
    // 假装放了点history上来当例子
    let history_items = vec![
    ("Hacking FBI server with raspberry pi", 1),
    ("COMPsci SICP tutorial course", 2),
    ("Proxy failure troubleshooting", 3),
    ("Wake me up when September ends chord", 4),
    ("Best OASIS songs top 100 all time", 5),
    ("Fix SSL/TLS Error", 6),
    ("React component quick fix", 7),
    ("Next.js 18 documentation", 8),
];


    let expanded_true = use_state(|| true);
    let toggle_expanded = {
        let expanded_true = expanded_true.clone();
        Callback::from(move |_| expanded_true.set(!*expanded_true))
    };

    html! {
        <div class={classes!("section", "history", if *expanded_true { "expanded" } else { "collapsed" })}>
            { if *expanded_true {
                html! { <div class="section-line"></div> }
            } else {
                html! {}
            }}
            <div class="section-header" onclick={toggle_expanded}>
                <div class="icon-line-wrapper">
                    <img src="assets/history.png" alt="History Icon" class="section-icon" />
                </div>
                <span class="section-title">{ "History" }</span>
                <img src={if *expanded_true { "assets/up.png" } else { "assets/down.png" }} alt="Toggle Arrow" class="section-arrow" />
            </div>
            { if *expanded_true {
                html! {
                    <div class="section-content">
                        <div class="history-content">
                            <ul class="history-list">
                                { for history_items.iter().map(|(item_name, item_id)| html! {
                                    <li class="history-item" key={item_id.to_string()}>
                                        <Link<Route> to={Route::HistoryItem { id: *item_id }} classes="history-link">
                                            { item_name }
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