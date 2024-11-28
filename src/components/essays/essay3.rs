use dioxus::prelude::*;

#[component]
pub fn Essay3() -> Element {
    rsx! {
        div {
            class: "ml-64 mt-16 mb-3",
            h1 {
                class: "font-display font-bold text-4xl text-black mb-4",
                "Essay 3: Ethics and Modern Thought"
            }
            p {
                class: "font-text text-lg text-gray-700 mb-4",
                "Examining ethical frameworks in contemporary philosophical discourse."
            }
            div {
                class: "font-text text-base text-gray-600",
                "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium."
            }
        }
    }
}