use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub active_route: Option<Route>,
}

#[function_component(History)]
pub fn history(_props: &Props) -> Html {
    let is_open = use_state(|| true);
    let on_click = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let chat_history = vec![
        ("The latest cover letter history", 1),
    ];

    let is_active = matches!(
        _props.active_route,
        Some(Route::History)
    );

    html! {
        <div class={classes!(
            "section",
            "history",
            if *is_open { "open" } else { "closed" },
            if is_active { "active" } else { "" }
        )}>
            { if *is_open {
                html! { <div class="divider"></div> }
            } else {
                html! {}
            }}
            
            <div class="section-header" onclick={on_click}>
                <div class="icon-wrap">
                    <img src="/assets/history.png" alt="History Icon" class="section-icon" />
                </div>
                <span class="section-title">{ "History" }</span>
                <img 
                    src={if *is_open { "/assets/up.png" } else { "/assets/down.png" }} 
                    alt="Toggle Arrow" 
                    class="section-arrow" 
                />
            </div>

            { if *is_open {
                html! {
                    <div class="section-content">
                        <div class="history-content">
                            <ul class="history-list">
                                { for chat_history.iter().map(|(name, _id)| {
                                    html! {
                                        <li class="chat_history">
                                            <Link<Route> 
                                                to={Route::History}
                                                classes={classes!(
                                                    "history-link",
                                                    if is_active { "active" } else { "" }
                                                )}
                                            >
                                                { name }
                                            </Link<Route>>
                                        </li>
                                    }
                                })}
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