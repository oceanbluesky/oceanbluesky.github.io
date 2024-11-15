use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            class: "flex justify-center", // Centers the navbar
            ul {
                class: "flex list-none p-0 m-0 gap-3 text-sm font-medium text-gray-800", // Slightly smaller, closer items, clear font
                li {
                    Link {
                        to: Route::Home {},
                        class: "no-underline px-3 py-2 hover:bg-blue-200 rounded-md transition-colors duration-200",
                        "Home"
                    }
                }
                li {
                    Link {
                        to: Route::About {},
                        class: "no-underline px-3 py-2 hover:bg-blue-200 rounded-md transition-colors duration-200",
                        "About"
                    }
                }
                li {
                    Link {
                        to: Route::EssaysList {},
                        class: "no-underline px-3 py-2 hover:bg-blue-200 rounded-md transition-colors duration-200",
                        "Essays"
                    }
                }
                li {
                    Link {
                        to: Route::Projects {},
                        class: "no-underline px-3 py-2 hover:bg-blue-200 rounded-md transition-colors duration-200",
                        "Projects"
                    }
                }
                li {
                    Link {
                        to: Route::Research {},
                        class: "no-underline px-3 py-2 hover:bg-blue-200 rounded-md transition-colors duration-200",
                        "Research"
                    }
                }
                li {
                    Link {
                        to: Route::Contact {},
                        class: "no-underline px-3 py-2 hover:bg-blue-200 rounded-md transition-colors duration-200",
                        "Contact"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
