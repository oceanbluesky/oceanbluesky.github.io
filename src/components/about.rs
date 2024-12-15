
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! { 
            div { 
                class: "about ml-64 mt-16 mb-3",
                h2 {
                    class: "font-display font-bold text-5xl text-black",
                    "About"
                }
                p {
                    class: "font-text text-lg text-gray-500", 
                    "TEST A brief biography goes here,", 
                    br {},
                    "describing career, skills, and interests." 
                }
                // img { src: "assets/images/profile.jpg", alt: "Profile Image" }
            }
     }
}
