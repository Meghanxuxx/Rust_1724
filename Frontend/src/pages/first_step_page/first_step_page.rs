use yew::prelude::*;
use crate::components::{Sidebar, Header};

#[function_component(FirstStepPage)]
pub fn first_step_page() -> Html {
    html! {
        <>
            <Header show_line={false} />
            <div class="first-step-page">
                <div class="app-content">
                    <Sidebar />
                    <div class="content-wrapper">
                        // 主要内容，还没写
                    </div>
                </div>
            </div>
        </>
    }
}
