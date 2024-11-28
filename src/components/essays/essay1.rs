use dioxus::prelude::*;

#[component]
pub fn Essay1() -> Element {
    rsx! {
        div {
            class: "essay ml-64 mt-16 mb-3",
            h2 {
                class: "font-display font-bold text-5xl text-black",
                "1.0.0: First Philosophical Exploration"
            }
            p {
                class: "font-text text-lg text-gray-500",
                "An exploration of fundamental philosophical concepts 
                delving into the core of intellectual inquiry."
            }
            div {
                class: "font-text text-base text-gray-600 mt-4",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            }
        }
    }
}