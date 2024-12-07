use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Steps)]
pub fn steps() -> Html {
    let location = use_location().unwrap();
    let current_route = location.path();

    let steps = vec![
        ("Step 1", Route::FirstStep, 1),
        ("Step 2", Route::SecondStep, 2),
        ("Step 3", Route::ThirdStep, 3),
        ("Summary", Route::FinalStep, 4),
    ];

    let expanded_true = use_state(|| true);
    let toggle_expanded = {
        let expanded_true = expanded_true.clone();
        Callback::from(move |_| expanded_true.set(!*expanded_true))
    };

    html! {
        <div class={classes!("section", "steps", if *expanded_true { "expanded" } else { "collapsed" })}>
            { if *expanded_true {
                html! { <div class="section-line"></div> }
            } else {
                html! {}
            }}
            <div class="section-header" onclick={toggle_expanded}>
                <div class="icon-line-wrapper">
                    <img src="assets/steps.png" alt="Steps Icon" class="section-icon" />
                </div>
                <span class="section-title">{ "Steps" }</span>
                <img src={if *expanded_true { "assets/up.png" } else { "assets/down.png" }} alt="Toggle Arrow" class="section-arrow" />
            </div>
            { if *expanded_true {
                html! {
                    <div class="section-content">
                        <ul class="section-list">
                            { for steps.iter().map(|(step_name, route, step_id)| {
                                let active_true = match route {
                                    Route::FirstStep => current_route == "/first-step",
                                    Route::SecondStep => current_route == "/second-step",
                                    Route::ThirdStep => current_route == "/third-step",
                                    Route::FinalStep => current_route == "/final-step",
                                    _ => false,
                                };
                                
                                html! {
                                    <li class="section-item" key={*step_id}>
                                        <Link<Route> 
                                            to={route.clone()} 
                                            classes={classes!("section-link")}
                                        >
                                            <span class={classes!("section-text", if active_true { "active-text" } else { "" })}>
                                                { step_name }
                                            </span>
                                        </Link<Route>>
                                    </li>
                                }
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