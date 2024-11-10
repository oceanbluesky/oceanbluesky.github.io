use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
use components::{about, essays, contact, footer, header, home, projects, research};

fn main() {
    dioxus::launch_with_props(App); // Pass the component function directly
}

fn App(cx: Scope) -> Element {
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
            Route { to: "/", home::Home {} }
            Route { to: "/about", about::About {} }
            Route { to: "/research", research::Research {} }
            Route { to: "/essays", essays::Essays {} }
            Route { to: "/contact", contact::Contact {} }
            Route { to: "/projects", projects::Projects {} }
        }
    })
}
