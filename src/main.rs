use dioxus::prelude::*;
use dioxus_router::{Router, Route, Link};


mod components;
use components::{about, essays, contact, home, projects, research};

fn main() {
    dioxus::launch(app); // Pass the component function directly
}

fn app(cx: ScopeState) -> Element {
    cx.render(rsx! {
        Router {
            nav {
                Link { to: "/", "Home" }
                Link { to: "/about", "About" }
                Link { to: "/research", "Research" }
                Link { to: "/essays", "Essays" }
                Link { to: "/projects", "Projects" }
                Link { to: "/contact", "Contact" }
            }
            Route { to: "/", component: components::home::Home {} }
            Route { to: "/about", component: components::about::About {} }
            Route { to: "/research", component: components::research::Research {} }
            Route { to: "/essays", component: components::essays::Essays {} }
            Route { to: "/contact", component: components::contact::Contact {} }
            Route { to: "/projects", component: components::projects::Projects {} }
        }
    })
}
