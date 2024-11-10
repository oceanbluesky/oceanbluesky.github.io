use dioxus::prelude::*;
use std::thread::Scope;

#[allow(non_snake_case)]
pub fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "projects",
            h2 { "Projects" }
            div { class: "gallery",
                figure {
                    img { src: "assets/images/project1.png", alt: "Project 1" }
                    figcaption { "Project 1 - Description" }
                }
                figure {
                    img { src: "assets/images/project2.png", alt: "Project 2" }
                    figcaption { "Project 2 - Description" }
                }
            }
        }
    })
}

/* 
    Showcase ML graphs, renders, or other projects as a gallery.
    Structure each project with an image and a brief description.
 */    