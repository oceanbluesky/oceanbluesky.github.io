
// src/components/essays_list.rs

// WHEN ADDING NEW ESSAYS, ADD THEM TO THE LIST BELOW, AND CREATE A NEW FILE IN THE ESSAYS FOLDER
// AND ADD THE COMPONENT AND ITS ROUTE TO MAIN.RS AND MOD.RS (three files need to be updated)

use dioxus::prelude::*;

#[component]
pub fn EssaysList() -> Element {
    rsx! {
        div {
            class: "essays ml-64 mt-16 mb-3",
            h2 {
                class: "font-display font-bold text-5xl text-black",
                "Essays"
            }
            p { 
                class: "font-text text-lg text-gray-500 mb-4", 
                "A poet should learn with their eyes
the forms of leaves.
They should know how to make people laugh when
all are together.
They should go to see what people are really like.
They should know about oceans and mountains
in themselves,
and the sun and the moon and the stars.
Their minds should enter into the seasons.
They should go among many people,
in many places,
and learn their languages.
Kshemendra, 12th Century" 
            }
            ul {
                class: "font-text text-lg text-gray-500", 
                li { a { href: "/essays/essay1", "Essay Title 1" } }
                li { a { href: "/essays/essay2", "Essay Title 2" } }
                li { a { href: "/essays/essay3", "Essay Title 3" } }
                li { a { href: "/essays/essay4", "Essay Title 4" } }
            }
        }
    }
}

