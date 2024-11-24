use dioxus::prelude::*;

#[component]
pub fn Research() -> Element {
    rsx! {
            div { 
                class: "reseaerch ml-64 mt-16 mb-3",
                h2 {
                    class: "font-display font-bold text-5xl text-black",
                    "Research"
                }
                p { 
                    class: "font-text text-lg text-gray-500 mb-3", 
                    "A brief message goes here." 
                }
                p { 
                    class: "font-text text-lg text-gray-500", 
                    "Research activities." 
                }
                // img { src: "assets/images/research.png", alt: "Research Image" }
            }
    }
}