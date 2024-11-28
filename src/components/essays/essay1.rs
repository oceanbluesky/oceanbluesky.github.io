use dioxus::prelude::*;

#[component]
pub fn Essay1() -> Element {
    rsx! {
        div {
            class: "ml-64 mt-16 mb-3",
            h1 {
                class: "font-display font-bold text-4xl text-black mb-4",
                "Essay 1: First Philosophical Exploration"
            }
            p {
                class: "font-text text-lg text-gray-700 mb-4",
                "This is the content of the first philosophical essay, exploring fundamental concepts and ideas."
            }
            div {
                class: "font-text text-base text-gray-600",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            }
        }
    }
}