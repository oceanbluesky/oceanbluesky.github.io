use dioxus::prelude::*;
use dioxus_router::prelude::*;


mod components;
use components::{about, essays, contact, footer, header, home, projects, research};

fn main() {
    dioxus::launch(App); // Pass the component function directly
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
            Route { to: "/", component: home::Home {} }
            Route { to: "/about", component: about::About {} }
            Route { to: "/research", component: research::Research {} }
            Route { to: "/essays", component: essays::Essays {} }
            Route { to: "/contact", component: contact::Contact {} }
            Route { to: "/projects", component: projects::Projects {} }
        }
    })
}
