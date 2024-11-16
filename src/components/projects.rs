use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
            div { 
                class: "projects ml-64 mt-16 mb-3",
                h2 {
                    class: "font-display font-bold text-5xl text-black",
                    "Projects"
                }
                p { 
                    class: "font-text text-xl text-gray-500 mb-4", 
                    "A brief description of what projects are goes here, describing career, skills, and interests." 
                }
                // img { src: "assets/images/projects.png", alt: "Projects Image" }
            }
            div { 
                class: "gallery ml-64 font-text text-x text-gray-500", 
                figure {
                    // img { src: "assets/images/project1.png", alt: "Project 1" }
                    figcaption { "Project 1 - Description" }
                }
                figure {
                    // img { src: "assets/images/project2.png", alt: "Project 2" }
                    figcaption { "Project 2 - Description" }
                }
            }
    }
}

/* 
    Showcase ML graphs, renders, or other projects as a gallery.
    Structure each project with an image and a brief description.
 */    