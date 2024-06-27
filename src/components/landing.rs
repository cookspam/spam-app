#[cfg(feature = "desktop")]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use ore_types::Transfer;
#[cfg(feature = "web")]
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    components::{ActivityFilter, ActivityIndicator, Footer, SpamIcon},
    gateway::AsyncResult,
    hooks::{use_is_onboarded, use_ore_supply, use_transfers, use_treasury},
    utils::asset_path,
    Route,
};


#[component]
pub fn Landing(cx: Scope) -> Element {
    let is_onboarded = use_is_onboarded(cx);
    let nav = use_navigator(cx);

    // If the user is already onboarded, redirect to home.
  //  if is_onboarded.read().0 {
    //    nav.replace(Route::Home {});
   // }

    render! {
        div {
            class: "flex flex-col",
            Hero{}
            Footer {hidden: false}
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
fn Navbar(cx: Scope) -> Element {
    render! {
        div {
            class: "max-w-[96rem] w-full flex flex-row justify-between mx-auto px-4 sm:px-8 py-6",
            Link {
                to: Route::Landing {},
                class: "flex flex-row h-10 gap-1",
                SpamIcon {
                    class: "w-6 h-6 my-auto "
                }
                p {
                    class: "text-2xl font-semibold my-auto",
                    "SPAM"
                }
            }
            div {
                class: "flex flex-row gap-6 md:gap-8 lg:gap-10",
                Link {
                    class: "transition-colors flex items-center gap-2 px-4 py-2 rounded-full text-black dark:text-gray-600 hover:text-gray-700 dark:hover:text-white",
                    to: Route::Home {},
                    span {
                        class: "text-2xl font-bold sm:text-xl sm:font-bold",
                        "⛏️"
                    }
                    span {
                        class: "text-sm font-semibold sm:text-base md:text-lg",
                        "Mine"
                    }
                }
                Link {
                    class: "transition-colors flex items-center gap-2 px-4 py-2 rounded-full text-black dark:text-gray-700 hover:text-gray-600 dark:hover:text-white",
                    to: Route::Stats {},
                    StatsIcon {}
                    span {
                        class: "text-sm font-semibold sm:text-base md:text-lg",
                        "Stats"
                    }
                }
                Link {
                    class: "transition-colors flex items-center gap-2 px-4 py-2 rounded-full text-black dark:text-gray-700 hover:text-gray-600 dark:hover:text-white",
                    to: Route::Settings {},
                    MyPageIcon {}
                    span {
                        class: "text-sm font-semibold sm:text-base md:text-lg",
                        "My Page"
                    }
                }
            }
        }
    }
}



#[component]
fn Hero(cx: Scope) -> Element {
    let bg_img = asset_path("spam_crop.png");
    let (treasury, _) = use_treasury(cx);
    let (supply, _) = use_ore_supply(cx);
    // TODO: calculate difficulty in number
    // let difficulty = match *treasury.read().unwrap() {
    //     AsyncResult::Ok(treasury) => {
    //         treasury.difficulty.to_string()
    //     }
    //     _ => " ".to_string(),
    // };
    let circulating_supply = match *treasury.read().unwrap() {
        AsyncResult::Ok(treasury) => {
            (treasury.total_claimed_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };
    
    let ore_supply = match supply {
        AsyncResult::Ok(token_amount) => token_amount.ui_amount.unwrap(),
        AsyncResult::Loading => 0f64,
        AsyncResult::Error(_err) => 0f64,
    };
    
    let reward_rate = match *treasury.read().unwrap() {
        AsyncResult::Ok(treasury) => {
            (treasury.reward_rate as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };
    let circulating_supply = format!("{:.2}", circulating_supply);
    let ore_supply = format!("{:.2}", ore_supply);
    let reward_rate = format!("{:.4}", reward_rate);

    render! {
        div {
            class: "bg-white max-w-[1280px] mx-auto",
            div {
                class: "relative w-full min-h-screen md:min-h-[100vh] z-20",
                Navbar {}
                div {
                    class: "relative flex flex-col items-center gap-y-8 sm:gap-y-10 md:gap-y-12 mx-auto pb-24 px-4 sm:px-8 max-w-7xl",
                    div {
                        class: "w-full h-screen",
                        img {
                            src: "{bg_img}",
                            class: "w-full h-auto object-scale-down m-auto"
                        }
                        div{
                            class: "absolute top-0 left-0 w-full",
                            div {
                                class: "pt-32 text-center text-7xl min-[480px]:text-5xl min-[600px]:text-6xl md:text-3xl lg:text-7xl font-bold font-hero text-black",
                                style: "text-shadow: 5px 5px 8px white, 0 0 50px white, 0 0 30px white;",
                              
                                    "Let's Spam Solana!"
                                
                            }
                        }
                        div {
                            class: "absolute -mt-[10vh] left-1/2 transform -translate-x-1/2 w-[86%] h-[20%] items-center flex flex-row justify-between bg-white p-4 rounded-lg shadow-md",
                            DataItem {
                                title: "Mining Difficulty".to_string(),
                                value: "1/256".to_string()
                            }
                            DataItem {
                                title: "Circulating Supply".to_string(),
                                value: circulating_supply
                            }
                            DataItem {
                                title: " Total Supply ".to_string(),
                                value: ore_supply 
                            }
                            DataItem {
                                title: "Reward Rate".to_string(),
                                value: reward_rate
                            }
                            div {
                                class: "flex flex-col items-center bg-teal-500 hover:bg-teal-700 rounded-lg p-4 shadow-md w-1/5 text-white text-lg ",
                              
                                    Link {
                                        class: " py-4 ",  // Adjusted padding and height
                                        to: Route::Home {},
                                        "SPAM →"
                                    }
                                
                            }
                        }
                        
                    }

                    

        
                }
            }
        }
    }
}


#[component]
fn DataItem(cx: Scope, title: String, value: String) -> Element {
    render! {
        div {
            class: "flex flex-col items-center bg-real_white rounded-lg p-4 shadow-md w-1/4",
            p {
                class: "text-gray-700 text-lg font-medium",
                "{title}"
            }
            p {
                class: "text-xl",
                "{value}"
            }
        }
    }
}


#[component]
fn OreValue(cx: Scope, title: String, amount: String) -> Element {
    render! {
        div {
            class: "flex flex-col gap-3",
            p {
                class: "text-gray-300 text-sm font-medium",
                "{title}"
            }
            div {
                class: "flex flex-row gap-2",
                SpamIcon {
                    class: "w-6 h-6 md:w-7 md:h-7 lg:w-8 lg:h-8 my-auto"
                }
                p {
                    class: "text-2xl md:text-3xl lg:text-4xl font-bold font-hero",
                    "{amount}"
                }
            }
        }
    }
}

#[component]
fn QuestionBreak(cx: Scope) -> Element {
    render! {
        div {
            class: "bg-green-500 text-white w-full py-16",
            p {
                class: "text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold font-hero text-center",
                "How much will you mine?"
            }
        }
    }
}
