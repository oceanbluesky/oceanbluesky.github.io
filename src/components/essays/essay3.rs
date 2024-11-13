use dioxus::prelude::*;

#[component]
pub fn Essay3(name: String) -> Element {
    rsx! { h2 { "Essay3: {name}" } }
}
