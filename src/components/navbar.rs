use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    let route = use_route::<Route>();

    // Helper function to determine if a route is active
    let is_active = move |check_route: Route| -> &'static str {
        if route == check_route {
            // Active link will have blue text without hover
            "no-underline px-3 py-2 text-blue-400 transition-colors duration-200"
        } else {
            // Inactive links will have blue text on hover
            "no-underline px-3 py-2 hover:text-blue-400 transition-colors duration-200"
        }
    };

    rsx! {
        nav {
            class: "flex justify-center mt-4",
            ul {
                class: "flex list-none p-0 m-0 gap-2 text-sm font-text font-light text-black",
                li {
                    Link {
                        to: Route::Home {},
                        class: is_active(Route::Home {}),
                        "Home"
                    }
                }
                li {
                    Link {
                        to: Route::About {},
                        class: is_active(Route::About {}),
                        "About"
                    }
                }
                li {
                    Link {
                        to: Route::EssaysList {},
                        class: is_active(Route::EssaysList {}),
                        "Essays"
                    }
                }
                li {
                    Link {
                        to: Route::Projects {},
                        class: is_active(Route::Projects {}),
                        "Projects"
                    }
                }
                li {
                    Link {
                        to: Route::Research {},
                        class: is_active(Route::Research {}),
                        "Research"
                    }
                }
                li {
                    Link {
                        to: Route::Contact {},
                        class: is_active(Route::Contact {}),
                        "Contact"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}