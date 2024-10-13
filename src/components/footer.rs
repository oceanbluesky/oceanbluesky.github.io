use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer",  // Apply the footer class
            "© 2025 copyright"
        }
    }
}