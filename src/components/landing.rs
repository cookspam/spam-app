#[cfg(feature = "desktop")]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{Footer, SpamIcon},
    gateway::AsyncResult,
    hooks::{use_is_onboarded, use_ore_supply, use_treasury},
    utils::asset_path,
    Route,
};


#[component]
pub fn Landing(cx: Scope) -> Element {
    // let is_onboarded = use_is_onboarded(cx);
    // let nav = use_navigator(cx);
    

    // If the user is already onboarded, redirect to home.
  //  if is_onboarded.read().0 {
    //    nav.replace(Route::Home {});
   // }

    render! {
        div {
            class: "flex flex-col",
            Hero{}
            Description {}
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
                class: "flex flex-row h-10 gap-1 my-auto",
                SpamIcon {
                    class: "w-5 h-5 sm:w-6 sm:h-6 my-auto"
                }
                p {
                    class: "text-lg sm:text-2xl font-semibold my-auto",
                    "SPAM"
                }
            }
            div {
                class: "flex flex-row gap-1 sm:gap-2 md:gap-4 lg:gap-10 flex-wrap sm:flex-nowrap",
                Link {
                    class: "transition-colors flex flex-col sm:flex-row items-center gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-black dark:text-gray-600 hover:text-gray-700 dark:hover:text-white",
                    to: Route::Home {},
                    span {
                        class: "w-5 h-5 sm:w-6 sm:h-6 my-auto",
                        "⛏️"
                    }
                    span {
                        class: "text-sm font-semibold sm:text-base md:text-lg",
                        "Mine"
                    }
                }
                Link {
                    class: "transition-colors flex flex-col sm:flex-row items-center gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-black dark:text-gray-600 hover:text-gray-700 dark:hover:text-white",
                    to: Route::Stats {},
                    StatsIcon {}
                    span {
                        class: "text-sm font-semibold sm:text-base md:text-lg",
                        "Stats"
                    }
                }
                Link {
                    class: "transition-colors flex flex-col sm:flex-row items-center gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-black dark:text-gray-600 hover:text-gray-700 dark:hover:text-white",
                    to: Route::Apps {},
                    span {
                        class: "w-5 h-5 sm:w-6 sm:h-6 my-auto",
                        "📦"
                    }
                    span {
                        class: "text-sm font-semibold sm:text-base md:text-lg",
                        "Apps"
                    }
                }
                Link {
                    class: "transition-colors flex flex-col sm:flex-row items-center gap-1 sm:gap-1 md:gap-2 lg:gap-3 px-2 sm:px-0 py-2 rounded-full text-black dark:text-gray-600 hover:text-gray-700 dark:hover:text-white",
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
                class: "relative w-full z-20 mb-20",
                Navbar {}
                div {
                    class: "relative flex flex-col items-center gap-y-8 sm:gap-y-10 md:gap-y-12 mx-auto pb-48 px-4 sm:px-8 max-w-7xl",
                    div {
                        class: "w-full",
                        img {
                            src: "{bg_img}",
                            class: "w-full h-auto object-scale-down m-auto"
                        }
                        div{
                            class: "absolute top-0 left-0 w-full",
                            div {
                                class: "text-center pt-32 text-3xl sm:text-6xl md:text-6xl lg:text-7xl font-bold font-hero text-black",
                                style: "text-shadow: 5px 5px 8px white, 0 0 50px white, 0 0 30px white;",  // Keep the shadow and other styles
                                "Let's Spam Solana!"
                            }
                            
                        }
                        div {
                            class: "absolute -mt-[20vh] left-1/2 transform -translate-x-1/2 w-[86%] h-[25%] items-center flex flex-col justify-between bg-white p-4 rounded-lg shadow-md",
                            
                            // DataItems section
                            div {
                                class: "flex flex-row justify-between w-full",  // Keep DataItems in a row on large screens
                                DataItem {
                                    title: "Mining Difficulty".to_string(),
                                    value: "1/256".to_string()
                                }
                                DataItem {
                                    title: "Circulating Supply".to_string(),
                                    value: circulating_supply
                                }
                                DataItem {
                                    title: "Total Supply".to_string(),
                                    value: ore_supply 
                                }
                                DataItem {
                                    title: "Reward Rate".to_string(),
                                    value: reward_rate
                                }
                            }
                            
                            // Button section
                            div {
                                class: "flex flex-col items-center bg-teal-500 hover:bg-teal-700 rounded-lg p-0 sm:p-4 md:p-4 lg:p-6 shadow-md w-full sm:w-auto lg:w-[19%] text-white text-lg mt-4 sm:mt-0", 
                                Link {
                                    class: "py-4 lg:py-2",  // Keep padding flexible for smaller screens and adjust on larger screens
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
            class: "flex flex-col items-center bg-real_white rounded-lg p-4 shadow-md w-1/4 mb-2",
            
            // Title with specific padding for top and left, reduced margin-bottom
            p {
                class: "text-sm sm:text-lg md:text-lg lg:text-lg text-gray-700 font-medium pt-2 pl-2 mb-1",  // Padding on top (pt) and left (pl), with smaller margin-bottom
                "{title}"
            }
            
            // Value with responsive font size and some margin-bottom for spacing
            p {
                class: "text-base sm:text-xl mb-2",  // Adjusted font size for value, with margin-bottom
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

#[component]
pub fn Description(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col",
            // New section starts here
            div {
                class: "text-center text-lg sm:text-2xl md:text-3xl lg:text-4xl font-bold mb-20 px-8",
                p {
                    class: "mt-8 mb-4",  
                    "Spam is a low-difficulty proof-of-work token, forked from ORE v1."
                }
                p {
                    class: "mb-8 ",  
                    "Anyone can mine to help stress-test and improve Solana's performance."
                }
            }
            div {
                class: "flex flex-col md:flex-row gap-8 max-w-7xl mx-auto px-4 sm:px-8 mb-20",
                
                div {
                    class: "flex-1 bg-white p-6 rounded-lg shadow-md mb-20",
                    h3 {
                        class: "text-xl font-semibold mb-8",
                        "Why Spam?"
                    }
                    ul {
                        class: "list-disc pl-5 pb-5",
                        li { "Boost Solana’s Network: Your mining efforts contribute directly to stress-testing and enhancing Solana’s blockchain stability." }
                        li { "Easy to Start: No complex setup required—just start mining with minimal resources." }
                        li { "Earn Rewards: Get rewarded for helping to optimize the Solana network." }
                    }
                }
                
                div {
                    class: "flex-1 bg-white p-6 rounded-lg shadow-md mb-20",
                    h3 {
                        class: "text-xl font-semibold mb-8",
                        "Start Mining in 3 Simple Steps:"
                    }
                    ul {
                        class: "list-disc pl-5",
                        li {
                          
                            "Set Up: "
                            a {
                                class: "text-teal-500 hover:text-teal-700",
                                href: "https://chromewebstore.google.com/detail/spam-chrome-extension/okchhmhmdibmceakkjehbfjipibmonbe",
                                "Install the SPAM Chrome Extension"
                            }
                            " or "
                            a {
                                class: "text-teal-500 hover:text-teal-700",
                                href: "https://spam.supply/apps",
                                "SPAM CLI"
                            }
                            "."
                        }
                        li { "Configure: Enter your mining key and select your preferred settings." }
                        li { "Mine: Your browser or CLI will automatically mine SPAM tokens, contributing to Solana's stress tests." }
                    }
                    
                }
                
                div {
                    class: "flex-1 bg-white p-6 rounded-lg shadow-md mb-20",
                    h3 {
                        class: "text-xl font-semibold mb-8",
                        "Get Started"
                    }
                    p {
                      
                        "Ready to join the SPAM network? - Choose your tool ⚒️:"
                        ul {
                            class: "pl-5 mt-2",
                            style: "list-style-type: circle; margin-left: 1rem;",
                            li {
                                "Start mining with "
                                a {
                                    class: "text-teal-500 hover:text-teal-700",
                                    href: "https://spam.supply/mine",
                                    "Web"
                                }
                            }
                            
                            li {
                                "Start mining with "
                                a {
                                    class: "text-teal-500 hover:text-teal-700",
                                    href: "https://chromewebstore.google.com/detail/spam-chrome-extension/okchhmhmdibmceakkjehbfjipibmonbe",
                                    "Chrome Extension"
                                }
                            }
                            
                            li {
                                "Start mining with "
                                a {
                                    class: "text-teal-500 hover:text-teal-700",
                                    href: "https://spam.supply/apps",
                                    "CLI"
                                }
                            }
                        }
                    }
                    
                    p {
                        class: "mt-4",
                        "Want to know more about how SPAM works? "
                        a {
                            class: "text-teal-500 hover:text-teal-700",
                            href: "https://spam.supply/spam-doc",
                            "Explore our FAQ and Documentation"
                        }
                    }
                }
            }
        }
    }
}