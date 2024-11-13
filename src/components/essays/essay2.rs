use dioxus::prelude::*;

#[component]
pub fn Essay2(name: String) -> Element {
    rsx! { h2 { "Essay2: {name}" } }
}
