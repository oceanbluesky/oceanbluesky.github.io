// src/components/essays/essay3.rs:

use dioxus::prelude::*;

#[component]
pub fn Essay3() -> Element {
    rsx! {
        div {
                class: "essay ml-64 mr-64 mt-16 mb-3",
                style: "margin-left: 16rem; margin-right: 16rem;",
            h1 {
                class: "font-display font-bold text-5xl text-black",
                "3.0.0: Ethics and Modern Thought"
            }
            p {
                class: "font-text text-lg text-gray-700 mb-4",
                "Examining ethical frameworks in contemporary philosophical discourse."
            }
            div {
                class: "font-text text-base text-gray-600 prose lg:prose-xl",
                p {
                    "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo."
                }
                p {
                    "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. "
                    strong { "Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet," }
                    " consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. "
                    em { "Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam," }
                    " nisi ut aliquid ex ea commodi consequatur?"
                }
                p {
                    "Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur? At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga."
                }
                div {
                    class: "image-container",
                    img {
                        src: "/images/ethics_diagram.png",
                        alt: "Diagram showing different ethical theories",
                        class: "w-full h-auto mb-4"
                    }
                    p {
                        "Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus."
                    }
                }
                p {
                    "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat."
                }
                div {
                    class: "math-container text-center",
                    p {
                        "Consider the ethical implications of the following mathematical model:"
                    }
                }
            }
        }
    }
}