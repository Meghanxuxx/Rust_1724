use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;
use std::process::id;

#[function_component(Steps)]
pub fn steps() -> Html {
    let steps = vec![
        ("Step 1", 1),
        ("Step 2", 2),
        ("Step 3", 3),
        ("Summary", 4),
    ];

    let is_expanded = use_state(|| true);
    let toggle_expanded = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| is_expanded.set(!*is_expanded))
    };

    html! {
        <div class={classes!("section", "steps", if *is_expanded { "expanded" } else { "collapsed" })}>
            { if *is_expanded {
                html! { <div class="section-line"></div> }
            } else {
                html! {}
            }}
            <div class="section-header" onclick={toggle_expanded}>
                <div class="icon-line-wrapper">
                    <img src="assets/steps.png" alt="Steps Icon" class="section-icon" />
                </div>
                <span class="section-title">{ "Steps" }</span>
                <img src={if *is_expanded { "assets/up.png" } else { "assets/down.png" }} alt="Toggle Arrow" class="section-arrow" />
            </div>
            { if *is_expanded {
                html! {
                    <div class="section-content">
                        <ul class="section-list">
                            { for steps.iter().map(|(step_name, step_id)| html! {
                                <li class="section-item" key={*step_id}>
                                    <Link<Route> to={Route::Step { id: *step_id }} classes="section-link">
                                        <span class="section-text">{ step_name }</span>
                                    </Link<Route>>
                                </li>
                            }) }
                        </ul>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}