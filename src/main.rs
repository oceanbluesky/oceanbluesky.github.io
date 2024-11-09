use dioxus::prelude::*;
use dioxus_router::{Route, Router, Link};

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            nav {
                Link { to: "/", "Home" }
                Link { to: "/about", "About" }
                Link { to: "/research", "Research" }
                Link { to: "/micro-blog", "Micro-Blog" }
                Link { to: "/contact", "Contact" }
                Link { to: "/projects", "Projects" }
            }
            Route { to: "/", home::Home {} }
            Route { to: "/about", about::About {} }
            Route { to: "/research", research::Research {} }
            Route { to: "/micro-blog", blog::Blog {} }
            Route { to: "/contact", contact::Contact {} }
            Route { to: "/projects", projects::Projects {} }
        }
    })
}