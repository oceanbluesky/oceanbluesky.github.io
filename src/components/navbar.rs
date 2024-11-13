use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "navbar",
            ul { class: "navbar__list",
                li { class: "navbar__item",
                    Link { to: Route::Home {}, "Home" }
                }
                li {class: "navbar__item",
                    Link { to: Route::About {}, "About" }
                }
                li {class: "navbar__item",
                    Link { to: Route::EssaysList {}, "Essays" }
                }
                li {class: "navbar__item",
                    Link { to: Route::Projects {}, "Projects" }
                }
                li {class: "navbar__item",
                    Link { to: Route::Research {}, "Research" }
                }
                li {class: "navbar__item",
                    Link { to: Route::Contact {}, "Contact" }
                }
            }
        }
        Outlet::<Route> {}
    }
}