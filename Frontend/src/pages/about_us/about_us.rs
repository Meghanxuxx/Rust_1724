use yew::prelude::*;
use crate::components::Header;

#[derive(Clone)]
struct Developer {
    name: &'static str,
    email: &'static str,
    avatar: &'static str,
}

#[function_component(AboutUs)]
pub fn about_us() -> Html {
    let developers = vec![
        Developer {
            name: "Xian Wu",
            email: "amandaxian.wu@mail.utoronto.ca",
            avatar: "assets/xianwu.jpg"
        },
        Developer {
            name: "Huilin Xu (Meghan)",
            email: "meghan.xu@mail.utoronto.ca",
            avatar: "assets/meghan.jpg"
        },
        Developer {
            name: "Shanzhi Zhang",
            email: "shanzhi.zhang@mail.utoronto.ca",
            avatar: "assets/shanzhi.jpg"
        }
    ];

    html! {
        <div class="page">
            <div class="team-page">
                <Header show_line={false} />
                <div class="team-content">
                    <h1 class="team-title">{"Meet Our Team"}</h1>
                    <p class="team-desc">{"The developers behind CoverDraft"}</p>
                    
                    <div class="team-members">
                        {
                            developers.into_iter().map(|dev| {
                                html! {
                                    <div class="member-box">
                                        <div class="member-pic">
                                            <img src={dev.avatar} alt={dev.name} class="pic" />
                                        </div>
                                        <h2 class="member-name">{dev.name}</h2>
                                        <a href={format!("mailto:{}", dev.email)} class="member-mail">
                                            {dev.email}
                                        </a>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
