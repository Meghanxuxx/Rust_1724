mod components;
mod pages;
mod types; 

use yew::prelude::*;
use yew_router::Routable;
use yew_router::BrowserRouter;
use components::Header;
use yew_router::Switch;
use pages::home_page::ContentSection;
use pages::first_step_page::FirstStepPage;
use pages::second_step_page::SecondStepPage;
use pages::third_step_page::ThirdStepPage;
use pages::create_account_page::CreateAccountPage;
use pages::login_page::LoginPage;
use pages::final_step_page::FinalStepPage;
use pages::about_us::AboutUs;
use pages::history_page::HistoryItemPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/create-account")]
    CreateAccount,
    #[at("/login")]
    Login,
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
    #[at("/final-step")]
    FinalStep,
    #[at("/about-us")]
    AboutUs,
    #[not_found]
    #[at("/404")]
    NotFound,
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
        Route::CreateAccount => html! {
            <CreateAccountPage />
        },
        Route::Login => html! {
            <LoginPage />
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
            <HistoryItemPage />
            // <p>{ format!("History item page with id: {}", id) }</p>
        },
        Route::FinalStep => html! {
            <FinalStepPage />
        },
        Route::AboutUs => html! {
            <AboutUs />
        },
        Route::NotFound => html! { <p>{ "Page Not Found x_x" }</p> },
    }
}