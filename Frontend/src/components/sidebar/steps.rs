// src/components/sidebar/steps.rs
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub active_route: Option<Route>,
}

#[function_component(Steps)]
pub fn steps(props: &Props) -> Html {
    let is_open = use_state(|| true);
    let on_click = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let is_active = |route: Route| -> bool {
        if let Some(active_route) = &props.active_route {
            *active_route == route
        } else {
            false
        }
    };

    let get_link_classes = |route: Route| -> Classes {
        classes!(
            "section-link",
            if is_active(route) { "active" } else { "" }
        )
    };

    html! {
        <div class={classes!(
            "section",
            "steps",
            if *is_open { "open" } else { "closed" }
        )}>
            { if *is_open {
                html! { <div class="divider"></div> }
            } else {
                html! {}
            }}
            <div class="section-header" onclick={on_click}>
                <div class="icon-wrap">
                    <img src="/assets/steps.png" alt="Steps Icon" class="section-icon" />
                </div>
                <span class="section-title">{ "Steps" }</span>
                <img 
                    src={if *is_open { "/assets/up.png" } else { "/assets/down.png" }} 
                    alt="Toggle Arrow" 
                    class="section-arrow" 
                />
            </div>
            { if *is_open {
                html! {
                    <div class="section-content">
                        <ul class="section-list">
                            <li class="section-item">
                                <Link<Route> to={Route::FirstStep} classes={get_link_classes(Route::FirstStep)}>
                                    { "Step 1" }
                                </Link<Route>>
                            </li>
                            <li class="section-item">
                                <Link<Route> to={Route::SecondStep} classes={get_link_classes(Route::SecondStep)}>
                                    { "Step 2" }
                                </Link<Route>>
                            </li>
                            <li class="section-item">
                                <Link<Route> to={Route::ThirdStep} classes={get_link_classes(Route::ThirdStep)}>
                                    { "Step 3" }
                                </Link<Route>>
                            </li>
                            <li class="section-item">
                                <Link<Route> to={Route::FinalStep} classes={get_link_classes(Route::FinalStep)}>
                                    { "Summary" }
                                </Link<Route>>
                            </li>
                        </ul>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}