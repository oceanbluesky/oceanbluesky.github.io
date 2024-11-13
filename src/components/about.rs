
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! { 
            div { 
                class: "about",
                h2 { "About Me" }
                p { "A brief biography goes here, describing career, skills, and interests." }
                img { src: "assets/images/profile.jpg", alt: "Profile Image" }
            }
     }
}
