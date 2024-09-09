use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{Banner, BannerType, Footer, SpamIcon},
    gateway::AsyncResult,
    hooks::{use_appearance, use_ping},
    route::Route,
};

use super::Appearance;

#[component]
pub fn Navbar(cx: Scope) -> Element {
    let ping = use_ping(cx);
    let appearance = use_appearance(cx);
    let dark = match *appearance.read() {
        Appearance::Dark => "dark",
        Appearance::Light => "",
    };
    let route = use_route::<Route>(cx);

    let hidden = if let Some(route) = route {
        matches!(
            route,
            Route::Home {} | Route::Tx { .. } | Route::User { .. }
        )
    } else {
        false 
    };
    render! {
        div {
            class: "relative min-h-screen flex flex-col text-black dark:bg-black dark:text-white {dark} overflow-hidden",
            if let AsyncResult::Error(_) = ping {
                render! {
                    Banner {
                        text: "Error connecting to Solana...".to_string(),
                        banner_type: BannerType::Error
                    }
                }
            }
            div {
                class: "flex w-full",
                div {
                    class: "max-w-[96rem] w-full flex flex-row justify-between mx-auto px-4 sm:px-8 py-6",  // Reduce py for smaller screens
                    Link {
                        to: Route::Landing {},
                        class: "flex flex-row h-10 gap-1 my-auto",
                        SpamIcon {
                            class: "w-5 h-5 sm:w-6 sm:h-6 my-auto"  // Smaller logo on small screens
                        }
                        p {
                            class: "text-lg sm:text-2xl font-semibold my-auto text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",  // Smaller text on small screens
                            "SPAM"
                        }
                    }
                    div {
                        class: "flex flex-row gap-1 sm:gap-2 md:gap-4 lg:gap-10 flex-wrap sm:flex-nowrap",  // Reduced gap for smaller screens
                        Link {
                            class: "transition-colors flex flex-col sm:flex-row items-center gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",  // Reduced padding
                            to: Route::Home {},
                            span {
                                class: "w-5 h-5 sm:w-6 sm:h-6 my-auto",  // Smaller icon on small screens
                                "⛏️"
                            }
                            span {
                                class: "text-xs sm:text-base md:text-lg",  // Smaller text on small screens
                                "Mine"
                            }
                        }
                        Link {
                            class: "transition-colors flex flex-col items-center sm:flex-row gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",  // Reduced padding
                            to: Route::Stats {},
                            StatsIcon {}
                            span {
                                class: "text-xs sm:text-base md:text-lg",  // Smaller text on small screens
                                "Stats"
                            }
                        }
                        Link {
                            class: "transition-colors flex flex-col items-center sm:flex-row gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-black dark:text-gray-700 hover:text-gray-600 dark:hover:text-white",  // Reduced padding
                            to: Route::Apps {},
                            span {
                                class: "w-5 h-5",  // Smaller icon on small screens
                                "📦"
                            }
                            span {
                                class: "text-xs sm:text-base md:text-lg",  // Smaller text on small screens
                                "Apps"
                            }
                        }
                        Link {
                            class: "transition-colors flex flex-col items-center sm:flex-row  gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",  // Reduced padding
                            to: Route::Settings {},
                            MyPageIcon {}
                            span {
                                class: "text-xs sm:text-base md:text-lg",  // Smaller text on small screens
                                "My Page"
                            }
                        }
                    }
                }
            }
            div { 
                class: "flex flex-col h-full py-4 px-4 sm:px-8 grow w-full max-w-[96rem] mx-auto",
                Outlet::<Route> {}
            }
            Footer {
                hidden: hidden
            }
        }
    }
}


#[component]
fn StatsIcon(cx: Scope) -> Element {
    render! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M4 6h16M4 12h16M4 18h16"
            }
        }
    }
}

#[component]
fn MyPageIcon(cx: Scope) -> Element {
    render! {
        span {
            class: "w-5 h-5 ",
            "👤"
        }
    }
}

#[component]
pub fn SimpleNavbar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col min-h-screen h-full bg-white text-black",
            div {
                class: "flex flex-row justify-between px-4 sm:px-8 py-8 w-full z-50",
                Link {
                    to: Route::Landing {},
                    class: "flex h-10",
                    SpamIcon{
                        class: "h-3 md:h-4 my-auto"
                    }
                }
            }
            div {
                class: "py-4 px-4 sm:px-8 grow h-full w-full max-w-[96rem] mx-auto",
                Outlet::<Route> {}
            }
            Footer {hidden:false}
        }
    }
}
