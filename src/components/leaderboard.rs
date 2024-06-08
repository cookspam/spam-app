use std::{rc::Rc, str::FromStr};

use dioxus::prelude::*;
use dioxus_router::prelude::*;
#[cfg(feature = "desktop")]
use solana_account_decoder::parse_token::UiTokenAccount;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "web")]
use solana_extra_wasm::account_decoder::parse_token::UiTokenAccount;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{OreIcon, Footer},
    gateway::{AsyncResult, Gateway},
    hooks::{use_gateway, use_ore_supply, use_treasury},
    route::Route,
    utils::asset_path,  // Add this line to use asset_path function
};

#[component]
pub fn Stats(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col min-h-screen", // Ensure it takes the full height of the screen
            div {
                class: "flex-grow", // Take remaining space
                div {
                    class: "flex flex-col gap-16 pt-10 pb-10",
                    SupplyStats {}
                    TopHolders {}
                }
            }
        }
    }
}

#[component]
pub fn SupplyStats(cx: Scope) -> Element {
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
            class: "flex flex-col gap-6 text-lg",
            h2 {
                class: "text-lg md:text-2xl font-bold",
                "Supply"
            }
            div {
                class: "flex flex-col gap-8 my-auto",
                OreValue {
                    title: "Circulating supply".to_string(),
                    detail: "The total amount of Ore that has been mined and claimed.".to_string(),
                    amount: circulating_supply
                }
                OreValue {
                    title: "Total supply".to_string(),
                    detail: "The total amount of Ore that has ever been mined.".to_string(),
                    amount: ore_supply
                }
            }
        }
    }
}

#[component]
fn OreValue(cx: Scope, title: String, detail: String, amount: String) -> Element {
    render! {
        div {
            class: "flex flex-row justify-between grow gap-8",
            div {
                class: "flex flex-col gap-1 my-auto",
                p {
                    class: "text-gray-300 font-medium my-auto text-black dark:text-white",
                    "{title}"
                }
                p {
                    class: "text-gray-300 text-sm",
                    "{detail}"
                }
            }
            div {
                class: "flex flex-row gap-1.5",
                OreIcon {
                    class: "w-4 h-4 my-auto"
                }
                p {
                    class: "font-medium my-auto",
                    "{amount}"
                }
            }
        }

    }
}

#[component]
pub fn TopHolders(cx: Scope) -> Element {
    let token_accounts = use_state(cx, || AsyncResult::Loading);
    let gateway = use_gateway(cx);
    let solo = asset_path("mining_solo.png");
    let search_query = use_state(cx, || "".to_string());

    use_future(cx, (), |_| {
        let gateway = gateway.clone();
        let token_accounts = token_accounts.clone();
        async move {
            token_accounts.set(AsyncResult::Ok(fetch_top_accounts(gateway).await));
        }
    });

    render! {
        div {
            class: "flex flex-col md:flex-row gap-16 relative border p-8 rounded-lg",  // Added border, padding, and rounded corners
            div {
                class: "flex flex-col flex-1",
                h2 {
                    class: "text-lg md:text-2xl font-bold mb-8",
                    "Top Holders"
                }
                
                match token_accounts.get() {
                    AsyncResult::Ok(token_accounts) => {
                        render! {
                            LeaderboardTable {
                                token_accounts: token_accounts,
                                search_query: search_query.get()
                            }
                        }
                    }
                    _ => render! {
                        div {
                            class: "flex flex-row w-full h-32 loading rounded",
                        }
                    }
                }
                // div {
                //     class: "relative mb-4 mt-4",  // Container for the search bar
                //     span {
                //         class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400",
                //         "🔍"  // Simple magnifying glass icon
                //     }
                //     input {
                //         class: "pl-10 pr-4 py-2 border-b border-gray-300 focus:outline-none focus:border-black w-full",
                //         r#type: "text",
                //         placeholder: "Search your address...",
                //         oninput: move |evt| {
                //             search_query.set(evt.value.clone());
                //         }
                //     }
                // }
            }
            div {
                class: "flex items-end",  // Align the image at the bottom
                img {
                    src: "{solo}",  // Adjust the image path accordingly
                    class: "w-48 h-48 md:w-64 md:h-64",
                    alt: "Mining Solo"
                }
            }
        }
    }
}

#[derive(Props)]
struct LeaderboardTableProps<'a> {
    token_accounts: &'a Vec<UiTokenAccount>,
    search_query: &'a str,
}

#[component]
pub fn LeaderboardTable<'a>(cx: Scope<'a, LeaderboardTableProps<'a>>) -> Element {
    let filtered_accounts: Vec<&UiTokenAccount> = cx.props.token_accounts.iter()
        .filter(|account| account.owner.contains(cx.props.search_query))
        .collect();

    if filtered_accounts.is_empty() {
        render! {
            p {
                class: "text-sm text-gray-300 py-2 px-1",
                "No transactions found"
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-col gap-2",
                div {
                    class: "flex flex-col gap-0 justify-start grow h-full",
                    LeaderboardTableHeader {}
                    for (i, token_account) in filtered_accounts.iter().enumerate() {
                        render! {
                            TokenBalanceRow {
                                i: i + 1,
                                token_account: token_account
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TokenBalanceRow<'a>(cx: Scope, i: usize, token_account: &'a UiTokenAccount) -> Element {
    let owner = if token_account.owner.eq(&ore::TREASURY_ADDRESS.to_string()) {
        "Ore Treasury".to_string()
    } else {
        token_account.owner.clone()
    };
    let amount = &token_account.token_amount.ui_amount_string;
    render! {
        Link {
            to: Route::User { id: token_account.owner.clone() },
            class: "flex flex-row shrink w-full justify-between rounded px-2 py-2 hover-100 active-200 transition-colors",
            p {
                class: "w-10 text-left",  // Reduce the width for #
                "{i}"
            }
            p {
                class: "w-3/5 text-left font-mono font-medium truncate",  // Increase the width for Account
                "{owner}"
            }
            div {
                class: "flex flex-row gap-1 w-1/5 text-right justify-end ",  // Decrease the width for Balance
                OreIcon {
                    class: "my-auto w-4 h-4"
                }
                p {
                    class: "font-medium",
                    "{amount}"
                }
            }
        }
    }
}

#[component]
pub fn LeaderboardTableHeader(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row shrink w-full justify-between rounded px-2 py-2 transition-colors text-xs font-medium text-gray-300",
            p {
                class: "text-left w-10",  // Adjusted width for number
                "#"
            }
            p {
                class: "text-left w-3/5",  // Adjusted width for account
                "Account"
            }
            p {
                class: "text-right w-1/5",  // Adjusted width for balance
                "Balance"
            }
        }
    }
}

// TODO Implement clone in solana-client-wasm to get this
// #[cached]
async fn fetch_top_accounts(gateway: Rc<Gateway>) -> Vec<UiTokenAccount> {
    let mut fetched_accounts = vec![];
    if let Ok(balances) = gateway.get_token_largest_accounts(&ore::MINT_ADDRESS).await {
        for balance in balances {
            if let Ok(pubkey) = Pubkey::from_str(&balance.address) {
                if let Ok(Some(token_account)) = gateway.get_token_account(&pubkey).await {
                    fetched_accounts.push(token_account);
                }
            }
        }
    }
    fetched_accounts
}
