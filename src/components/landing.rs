#[cfg(feature = "desktop")]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use ore_types::Transfer;
#[cfg(feature = "web")]
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    components::{ActivityFilter, ActivityIndicator, Footer, OreIcon, OreLogoIcon},
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
    if is_onboarded.read().0 {
        nav.replace(Route::Home {});
    }

    render! {
        div {
            class: "flex flex-col",
            Hero{}
            Block {
                title: "Proof of work.",
                title2: "On Solana.",
                detail: "Ore uses a novel mining protocol designed for fair token distribution. It guarantees no miner can ever be starved out from earning rewards.",
                section: Section::A
            }
            Block {
                title: "Stable supply.",
                title2: "Steady growth.",
                detail: "Ore has an algorithmic supply programmed for constant linear growth. On average, one new Ore token is mined every minute by miners around the globe.",
                section: Section::B
            }
            Footer {}
        }
    }
}

#[component]
fn Navbar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row justify-between px-4 sm:px-8 py-8 w-full z-50 bg-white",
            Link {
                to: Route::Landing {},
                class: "flex flex-row h-10",
                OreLogoIcon {
                    class: "h-6 md:h-8"
                }
            }
            div {
                class: "flex flex-row gap-4",
                Link {
                    to: Route::WhatIsMining {},
                    class: "text-lg font-semibold",
                    "Help"
                }
                Link {
                    to: Route::OreTokenomics {},
                    class: "text-lg font-semibold pr6",
                    "My Page"
                }
            }
        }
    }
}

#[component]
fn Hero(cx: Scope) -> Element {
    let bg_img = asset_path("spam(1).jpg");
    render! {
        div {
            class: "bg-white",
            div {
                class: "relative w-full min-h-screen md:min-h-[180vh] z-20",
                Navbar {}
                div {
                    class: "relative flex flex-col items-center gap-y-8 sm:gap-y-10 md:gap-y-12 mx-auto pb-24 px-4 sm:px-8 max-w-7xl",
                    div {
                        class: "w-full h-auto max-w-[1200px]",
                        img {
                            src: "{bg_img}",
                            class: "w-full h-auto"
                        }
                        div {
                            class: "absolute bottom-[-12.5%] left-1/2 transform -translate-x-1/2 w-[86%] h-[25%] items-center flex flex-row justify-between bg-white p-4 rounded-lg shadow-md",
                            DataItem {
                                title: "Live Tx",
                                value: "Dummy Value" // Dummy value
                            }
                            DataItem {
                                title: "Circulating Supply",
                                value: "0 SPAM" // Dummy value
                            }
                            DataItem {
                                title: "Total Supply",
                                value: "0 SPAM" // Dummy value
                            }
                            DataItem {
                                title: "Reward Rate",
                                value: "0.001324 SPAM" // Dummy value
                            }
                            div {
                                class: "flex flex-col justify-center h-full",
                                div {
                                    class: "flex flex-col items-center justify-center bg-teal-500 hover:bg-teal-700 text-white transition-colors rounded-lg shadow-md",
                                    Link {
                                        class: "px-12 py-5 w-80",  // Adjusted padding and height
                                        to: Route::Home {},
                                        "SPAM →"
                                    }
                                }
                            }
                        }
                        
                    }

                    

                    div {
                        class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8 items-center pt8 mt-24", // Added margin to separate the text from the data section
                        p {
                            class: "text-center text-2xl min-[480px]:text-5xl min-[600px]:text-6xl md:text-7xl lg:text-8xl font-bold font-hero text-black",
                            "Let's Spam Solana!"
                        }
                        div {
                            class: "w-full flex justify-center",
                            p {
                                class: "text-center text-xl sm:text-2xl md:text-3xl lg:text-4xl max-w-[46rem] font-hero leading-7 text-black pb-5",
                                "Contribute to the resilience of the Solana network while earning your share."
                            }
                        }
                    }
                }
            }
        }
    }
}


#[component]
fn DataItem(cx: Scope, title: &'static str, value: &'static str) -> Element {
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
fn Block<'a>(
    cx: Scope,
    title: &'a str,
    title2: Option<&'a str>,
    detail: &'a str,
    section: Section,
) -> Element {
    let colors = match section {
        Section::A => "bg-black text-white",
        Section::B => "bg-white text-black",
    };
    let height = match section {
        Section::A | Section::B => "min-h-svh h-full",
    };
    render! {
        div {
            class: "flex w-full z-20 {colors} {height}",
            div {
                class: "flex flex-col h-full w-full py-16 gap-24 px-4 sm:px-8",
                div {
                    class: "flex flex-col gap-4 sm:gap-6 md:gap-8",
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                        "{title}"
                        if let Some(title2) = title2 {
                            render! {
                                br{}
                                span {
                                    class: "opacity-50",
                                    "{title2}"
                                }
                            }
                        }
                    }
                    p {
                        class: "text-lg sm:text-xl md:text-2xl lg:text-3xl leading-relaxed max-w-[48rem] font-hero",
                        "{detail}"
                    }
                    BlockCta {
                        section: section
                    }
                }
                div {
                    class: "flex h-full w-full",
                    match section {
                        Section::A => render! { SectionA {} },
                        Section::B => render! { SectionB {} },
                    }
                }
            }
        }
    }
}

#[component]
fn BlockCta<'a>(cx: Scope, section: &'a Section) -> Element {
    match section {
        Section::A => render! {
            Link {
                class: "font-semibold mt-4",
                to: Route::WhatIsMining {},
                "Learn more →"
            }
        },
        Section::B => render! {
            Link {
                class: "font-semibold mt-4",
                to: Route::OreTokenomics {},
                "Learn more →"
            }
        },
    }
}

#[derive(PartialEq, Eq)]
enum Section {
    A,
    B,
}

#[component]
fn SectionA(cx: Scope) -> Element {
    let filter = use_state(cx, || ActivityFilter::Global);
    let offset = use_state(cx, || 0);
    let (transfers, _) = use_transfers(cx, filter, offset);

    render! {
        div {
            class: "flex flex-col w-full my-auto gap-4 max-w-[48rem]",
            div {
                class: "flex flex-row gap-2",
                ActivityIndicator {}
                p {
                    class: "font-semibold text-xl opacity-50",
                    "Live transactions"
                }
            }
            div {
                class: "flex flex-col w-full",
                TransfersSection {
                    transfers: transfers
                }
            }
        }
    }
}

#[component]
fn TransfersSection(cx: Scope, transfers: AsyncResult<Vec<Transfer>>) -> Element {
    match transfers {
        AsyncResult::Ok(transfers) => {
            if transfers.is_empty() {
                render! {
                    p {
                        class: "text-sm opacity-50",
                        "No transactions yet"
                    }
                }
            } else {
                render! {
                    for (i, transfer) in transfers.iter().enumerate() {
                        if i.lt(&5) {
                            let addr = transfer.to_address[..5].to_string();
                            let amount = (transfer.amount as f64) / (10f64.powf(ore::TOKEN_DECIMALS as f64));

                            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                            let ts = Duration::from_secs(transfer.ts as u64);
                            let time = now.saturating_sub(ts);
                            let t = time.as_secs();
                            const ONE_MIN: u64 = 60;
                            const ONE_HOUR: u64 = ONE_MIN * 60;
                            const ONE_DAY: u64 = ONE_HOUR * 24;
                            let time_str = if t.gt(&ONE_DAY) {
                                format!("{}d ago", t.saturating_div(ONE_DAY))
                            } else if t.gt(&ONE_HOUR) {
                                format!("{}h ago", t.saturating_div(ONE_HOUR))
                            } else if t.gt(&ONE_MIN) {
                                format!("{}m ago", t.saturating_div(ONE_MIN))
                            } else {
                                format!("{}s ago", t)
                            };

                            render! {
                                div {
                                    class: "flex flex-row py-3 gap-3 w-full transition-colors rounded hover:bg-gray-900 px-2 -mx-2",
                                    div {
                                        class: "flex flex-col pt-1",
                                        p {
                                            class: "flex flex-row gap-2",
                                            span {
                                                class: "font-mono font-bold",
                                                "{addr}"
                                            }
                                            "mined "
                                            span {
                                                class: "flex flex-row font-semibold gap-0.5",
                                                OreIcon {
                                                    class: "w-3.5 h-3.5 my-auto",
                                                }
                                                "{amount:.4}"
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex pt-1.5 ml-auto",
                                        p {
                                            class: "opacity-50 text-right text-nowrap text-sm",
                                            "{time_str}"
                                        }
                                    }
                                }
                            }
                        } else {
                            None
                        }
                    }
                }
            }
        }
        _ => None,
    }
}

#[component]
fn SectionB(cx: Scope) -> Element {
    let (treasury, _) = use_treasury(cx);
    let (supply, _) = use_ore_supply(cx);
    let circulating_supply = match *treasury.read().unwrap() {
        AsyncResult::Ok(treasury) => {
            (treasury.total_claimed_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    }
    .to_string();
    let ore_supply = match supply {
        AsyncResult::Ok(token_amount) => token_amount.ui_amount.unwrap().to_string(),
        AsyncResult::Loading => "-".to_string(),
        AsyncResult::Error(_err) => "Err".to_string(),
    };
    render! {
        div {
            class: "flex flex-col gap-12 my-auto",
            OreValue {
                title: "Circulating supply".to_string(),
                amount: circulating_supply
            }
            OreValue {
                title: "Total supply".to_string(),
                amount: ore_supply
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
                OreIcon {
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
