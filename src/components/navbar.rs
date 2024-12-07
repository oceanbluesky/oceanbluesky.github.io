use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            class: "flex justify-center mt-4",
            ul {
                class: "flex list-none p-0 m-0 gap-2 text-sm font-text font-light text-gray-500",
                li {
                    Link {
                        to: Route::Home {},
                        class: "no-underline px-3 py-2 hover:text-blue-400 transition-colors duration-200",
                        "HOME"
                    }
                }
                li {
                    Link {
                        to: Route::About {},
                        class: "no-underline px-3 py-2 hover:text-blue-400 transition-colors duration-200",
                        "ABOUT"
                    }
                }
                li {
                    Link {
                        to: Route::EssaysList {},
                        class: "no-underline px-3 py-2 hover:text-blue-400 transition-colors duration-200",
                        "ESSAYS"
                    }
                }
                li {
                    Link {
                        to: Route::Projects {},
                        class: "no-underline px-3 py-2 hover:text-blue-400 transition-colors duration-200",
                        "PROJECTS"
                    }
                }
                li {
                    Link {
                        to: Route::Research {},
                        class: "no-underline px-3 py-2 hover:text-blue-400 transition-colors duration-200",
                        "RESEARCH"
                    }
                }
                li {
                    Link {
                        to: Route::Contact {},
                        class: "no-underline px-3 py-2 hover:text-blue-400 transition-colors duration-200",
                        "CONTACT"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}