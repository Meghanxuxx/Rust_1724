use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Steps)]
pub fn steps() -> Html {
    let location = use_location().unwrap();
    let current_route = location.path();

    let steps = vec![
        ("Step 1", Route::FirstStep, 1),
        ("Step 2", Route::Step { id: 2 }, 2),
        ("Step 3", Route::Step { id: 3 }, 3),
        ("Summary", Route::Step { id: 4 }, 4),
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
                            { for steps.iter().map(|(step_name, route, step_id)| {
                                let is_active = match route {
                                    Route::FirstStep => current_route == "/first-step",
                                    Route::Step { id } => current_route == format!("/step/{}", id),
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