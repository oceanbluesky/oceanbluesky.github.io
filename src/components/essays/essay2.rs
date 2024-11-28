use dioxus::prelude::*;

#[component]
pub fn Essay2() -> Element {
    rsx! {
        div {
            class: "essay ml-64 mt-16 mb-3",
            h1 {
                class: "font-display font-bold text-4xl text-black mb-4",
                "2.0.0: Contemplations on Existence"
            }
            p {
                class: "font-text text-lg text-gray-700 mb-4",
                "A deep dive into the nature of being, consciousness, and philosophical inquiry."
            }
            div {
                class: "font-text text-base text-gray-600",
                "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."
            }
        }
    }
}