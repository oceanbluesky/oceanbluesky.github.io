use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            ul { class: "flex list-none p-0 m-0 gap-4",
                li {
                    Link {
                        to: Route::Home {},
                        class: "no-underline text-inherit px-4 py-2 hover:bg-gray-200 rounded-md",
                        "Home"
                    }
                }
                li {
                    Link {
                        to: Route::About {},
                        class: "no-underline text-inherit px-4 py-2 hover:bg-gray-200 rounded-md",
                        "About"
                    }
                }
                li {
                    Link {
                        to: Route::EssaysList {},
                        class: "no-underline text-inherit px-4 py-2 hover:bg-gray-200 rounded-md",
                        "Essays"
                    }
                }
                li {
                    Link {
                        to: Route::Projects {},
                        class: "no-underline text-inherit px-4 py-2 hover:bg-gray-200 rounded-md",
                        "Projects"
                    }
                }
                li {
                    Link {
                        to: Route::Research {},
                        class: "no-underline text-inherit px-4 py-2 hover:bg-gray-200 rounded-md",
                        "Research"
                    }
                }
                li {
                    Link {
                        to: Route::Contact {},
                        class: "no-underline text-inherit px-4 py-2 hover:bg-gray-200 rounded-md",
                        "Contact"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
