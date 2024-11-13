use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "not-found",
            h1 { "Page Not Found" }
            p { "Sorry, the page you requested doesn't exist." }
            pre { color: "red", "Attempted to navigate to: {route:?}" }
        }
    }
}