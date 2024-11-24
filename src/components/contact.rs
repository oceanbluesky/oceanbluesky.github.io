use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! { 
        div { 
            class: "contact ml-64 mt-16 mb-3",
            h2 {
                class: "font-display font-bold text-5xl text-black",
                "Contact"
            }
            p {
                class: "font-text text-lg text-gray-500", 
                "Feel free to reach out via the following platforms."
            }
                div {
                    class: "flex space-x-4 mt-4 gap-2 text-sm font-text font-light text-black",
                    a {
                        href: "https://github.com/oceanbluesky",
                        target: "_blank",
                        class: "font-text", 
                        "GitHub"
                    }
                    a {
                        href: "https://linkedin.com/in/ericmachmer",
                        target: "_blank",
                        class: "font-text",
                        "LinkedIn"
                    }
            }
        }
    }
}