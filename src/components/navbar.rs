use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{Banner, BannerType, Footer, OreLogoIcon},
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
            class: "relative min-h-screen flex flex-col text-black dark:bg-black dark:text-white {dark}",
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
                    class: "max-w-[96rem] w-full flex flex-row justify-between mx-auto px-4 sm:px-8 py-6",
                    Link {
                        to: Route::Landing {},
                        class: "flex h-10",
                        p {
                            class: "text-2xl font-semibold my-auto",
                            "SPAM"
                        }
                    }
                    div {
                        class: "flex flex-row gap-6 md:gap-8 lg:gap-10",
                        Link {
                            class: "transition-colors flex items-center gap-2 px-4 py-2 rounded-full text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",
                            to: Route::Home {},
                            span {
                                class: "text-2xl font-bold sm:text-xl sm:font-bold",
                                "⛏️"
                            }
                            span {
                                class: "text-sm sm:text-base md:text-lg",
                                "Mine"
                            }
                        }
                        Link {
                            class: "transition-colors flex items-center gap-2 px-4 py-2 rounded-full text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",
                            to: Route::Stats {},
                            StatsIcon {}
                            span {
                                class: "text-sm sm:text-base md:text-lg",
                                "Stats"
                            }
                        }
                        Link {
                            class: "transition-colors flex items-center gap-2 px-4 py-2 rounded-full text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",
                            to: Route::Settings {},
                            MyPageIcon {}
                            span {
                                class: "text-sm sm:text-base md:text-lg",
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
            class: "w-5 h-5 sm:w-6 sm:h-6",
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
            class: "w-5 h-5 sm:w-6 sm:h-6",
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
                    class: "flex flex-row h-10",
                    p {
                        class: "text-2xl font-semibold my-auto",
                        "SPAM"
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
