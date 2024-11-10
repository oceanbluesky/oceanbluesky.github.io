
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "about",
            h2 { "About Me" }
            p { "A brief biography goes here, describing career, skills, and interests." }
            img { src: "assets/images/profile.jpg", alt: "Profile Image" }
        }
    })
}
