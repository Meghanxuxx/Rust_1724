mod components;
mod pages;
mod api;

use yew::prelude::*;
use yew_router::Routable;
use yew_router::BrowserRouter;
use components::Header;
use yew_router::Switch;
use pages::home_page::ContentSection;
use pages::first_step_page::FirstStepPage;
use pages::second_step_page::SecondStepPage;
use pages::third_step_page::ThirdStepPage;
use pages::user_check_page::UserCheckPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/first-step")]
    FirstStep,
    #[at("/second-step")]
    SecondStep,
    #[at("/third-step")]
    ThirdStep,
    #[at("/step/:id")]
    Step { id: usize },
    #[at("/history/:id")]
    HistoryItem { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/user-check")]
    UserCheck,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <Header show_line=true />
                <div class="content-wrapper">
                    <ContentSection />
                </div>
            </>
        },
        Route::FirstStep => html! {
            <FirstStepPage />
        },
        Route::SecondStep => html! {
            <SecondStepPage />
        },
        Route::ThirdStep => html! {
            <ThirdStepPage />
        },
        Route::Step { id } => html! {
            <p>{ format!("Step page with id: {}", id) }</p>
        },
        Route::HistoryItem { id } => html! {
            <p>{ format!("History item page with id: {}", id) }</p>
        },
        Route::NotFound => html! { <p>{ "Page Not Found x_x" }</p> },
        Route::UserCheck => html! {
            <UserCheckPage />
        },
    }
}