use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    rsx! { 
            div { 
                class: "welcome ml-64 mt-16 mb-3",
                h2 {
                    class: "font-display font-bold text-5xl text-black",
                    "Welcome"
                }
                p { 
                    class: "font-text text-xl text-gray-500", 
                    "A brief message goes here." 
                }
                // img { src: "assets/images/welcome.png", alt: "Welcome Image" }
            }
        }
}
