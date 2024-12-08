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

    let is_open = use_state(|| true);
    let on_click = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class={classes!("section", "steps", if *is_open { "expanded" } else { "collapsed" })}>
            { if *is_open {
                html! { <div class="divider"></div> }
            } else {
                html! {}
            }}
            <div class="section-header" onclick={on_click}>
                <div class="icon-wrap">
                    <img src="assets/steps.png" alt="Steps Icon" class="section-icon" />
                </div>
                <span class="section-title">{ "Steps" }</span>
                <img src={if *is_open { "assets/up.png" } else { "assets/down.png" }} alt="Toggle Arrow" class="section-arrow" />
            </div>
            { if *is_open {
                html! {
                    <div class="section-content">
                        <ul class="section-list">
                            { for steps.iter().map(|(step_name, route, step_id)| {
                                let is_active = match route {
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
                                            <span class={classes!("section-text", if is_active { "active-text" } else { "" })}>
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