use std::{rc::Rc, str::FromStr,};
use std::string::String;
use web_sys::window;
//use plotters::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;
use chrono::{NaiveDateTime, Datelike, Timelike};
use web_time::{SystemTime, Duration, UNIX_EPOCH};

use reqwest::Client;
use serde::Deserialize;

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
    components::{SpamIcon},
    gateway::{ore_token_account_address, AsyncResult, Gateway, GatewayError, proof_pubkey},
    hooks::{use_gateway, use_ore_supply, use_treasury},
    route::Route,
    utils::asset_path,  // Add this line to use asset_path function
};
use ore::{state::Proof, utils::AccountDeserialize};


            
#[component]
pub fn Stats(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col min-h-screen", // Ensure it takes the full height of the screen
            div {
                class: "flex-grow", // Take remaining space
                div {
                    class: "flex flex-col gap-16 pt-10 pb-10",
                  //  ActiveUser{}
                    SupplyStats {}
                    QuerySpamBalance {}
                    TopHolders {}
                }
            }
        }
    }
}

#[derive(Deserialize, Clone)]
struct TransactionCount {
    count: u32,
    timestamp: String,
}

#[derive(Clone)]
struct TransactionWithHeight {
    count: u32,
    timestamp: String,
    height: f64,
}

async fn fetch_transaction_counts(url: &str) -> Result<Vec<TransactionCount>, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let data = response.json::<Vec<TransactionCount>>().await?;
    Ok(data)
}

#[component]
pub fn SupplyStats(cx: Scope) -> Element {

    let transaction_counts = use_state(&cx, Vec::new);
    let max_count = use_state(&cx, || 200);
    let selected_option = use_state(&cx, || "hourly");
    //let selected_option = use_state(&cx, || "hourly".to_string());
    let count_sum = use_state(&cx, || 0);
    let show_dropdown = use_state(&cx, || false);
    let is_fetched = use_state(&cx, || false); // Flag to track if data is fetched
    
    
    use_future(&cx, selected_option, |selected_option| {
        to_owned![transaction_counts, max_count, selected_option, count_sum, is_fetched];
        async move {
            is_fetched.set(false);
            transaction_counts.set(Vec::new());
            let url = match *selected_option.get() {
                "daily" => {
                    let now = SystemTime::now();
                    let to = now.duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    let from = (now - Duration::from_secs(7 * 24 * 3600)).duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    format!("https://transactionscountdaily-uud64dt76q-uc.a.run.app/?from={}&to={}", from, to)
                }
                "daily_30" => {
                    let now = SystemTime::now();
                    let to = now.duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    let from = (now - Duration::from_secs(30 * 24 * 3600)).duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    format!("https://transactionscountdaily-uud64dt76q-uc.a.run.app/?from={}&to={}", from, to)
                }
                "weekly" => {
                    let now = SystemTime::now();
                    let to = now.duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    let from = (now - Duration::from_secs(90 * 24 * 3600)).duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    format!("https://transactionscountweekly-uud64dt76q-uc.a.run.app/?from={}&to={}", from, to)
                }
                "monthly" => {
                    let now = SystemTime::now();
                    let to = now.duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    let from = (now - Duration::from_secs(365 * 24 * 3600)).duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    format!("https://transactionscountmonthly-uud64dt76q-uc.a.run.app/?from={}&to={}", from, to)
                }
                _ => {
                    let now = SystemTime::now();
                    let to = now.duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
                    let from = (now - Duration::from_secs(125 * 360)).duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
        
                    format!("https://transactionscounthourly-uud64dt76q-uc.a.run.app/?from={}&to={}", from, to)
                }
            };
           
            match fetch_transaction_counts(&url).await {
                Ok(data) => {
                    let max: u32 = data.iter().map(|tx| tx.count).max().unwrap_or(200);
                    max_count.set(max);
                    let total_count: u32 = data.iter().map(|tx| tx.count).sum();
                    count_sum.set(total_count);
                    // console::log_1(&format!("max {}", max).into());
                    //console::log_1(&format!("max_count {}", *max_count.get()).into());

                    let transformed_data: Vec<TransactionWithHeight> = data.iter().map(|tx| {
                        let date_time = NaiveDateTime::parse_from_str(&tx.timestamp, "%Y-%m-%dT%H:%M:%S%.fZ").unwrap();
                       
                        let formatted_timestamp = match *selected_option.get() {
                            "daily" | "daily_30" => format!("{:02}/{:02}", date_time.month(), date_time.day()),
                            "weekly" => format!("{:02}/{:02}", date_time.month(), date_time.day()),
                            "monthly" => format!("{}", date_time.format("%B")), // Full month name
                            _ => {
                                    let mut hourly_hour = date_time.hour();
                                    if date_time.minute() != 0 {
                                        hourly_hour += 1;
                                        if hourly_hour >= 24 {
                                            hourly_hour -= 24;
                                        }
                                    }
                                    format!("{:02}:{:02}", hourly_hour, 0)

                                }, // hourly
                        };
                        TransactionWithHeight {
                            count: tx.count,
                            timestamp: formatted_timestamp,
                            height: (tx.count as f64 / max as f64) * 100.0,
                            //total_count: total_count as f64
                        }
                    }).collect();
                    transaction_counts.set(transformed_data.clone());
                   is_fetched.set(true); // Mark data as fetched
                },
                Err(err) => console::log_1(&format!("Error fetching transaction counts: {}", err).into()),
            }
        }
    });
    let recent_tx_counts: Vec<u32> = transaction_counts
    .get()
    .iter()
    .rev()  // Reverse the iterator to get the last items first
    .take(2)  // Take the last 4 hours' counts
    .map(|tx| tx.count)
    .collect();

    // Log the last transaction count to the console
  
    console::log_1(&format!("recent_last_tx_count: {:?}", recent_tx_counts).into());

    let screen_size = is_small_screen();
    console::log_1(&format!("is_small_screen: {}", screen_size).into());

    let average_tx_count: u32 = if recent_tx_counts.len() == 2 {
        recent_tx_counts.iter().sum::<u32>() / 2
    } else {
        0
    };

    
    // Log the average of the last 4 hours to the console
    console::log_1(&format!("recent_tx_counts: {:?}", recent_tx_counts).into());
    console::log_1(&format!("average_tx_count: {}", average_tx_count).into());
    
    // Check if the average of the last 4 transactions is less than 1500
  
    let recent_low_tx_count = *is_fetched.get() && average_tx_count < 1500;


    
    let (treasury, _) = use_treasury(cx);
    let (supply, _) = use_ore_supply(cx);
    let circulating_supply = match *treasury.read().unwrap() {
        AsyncResult::Ok(treasury) => {
            (treasury.total_claimed_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    };
   
    let spam_supply = match supply {
        AsyncResult::Ok(token_amount) => token_amount.ui_amount.unwrap(),
        AsyncResult::Loading => 0f64,
        AsyncResult::Error(_err) => 0f64,
    };

    let pie = (circulating_supply as f64) / (spam_supply as f64) * 100.0;
    let remaining_pie = 100.0 - pie;
    let remaining_spam_text = format!("{:.2} %", remaining_pie);

    let ITEMS_PER_PAGE = match *selected_option.get() {
        "daily" => 7,
        "daily_30" => 30,
        "weekly" => 12,//
        "monthly" => 12,
        _ => 12, // hourly
    };

    let page_data: Vec<_> = transaction_counts.iter()
    .take(ITEMS_PER_PAGE)
    .collect();

    let y_max = ((*max_count.get() as f64) / 1000.0).ceil() * 1000.0;
    let y_20 = (y_max as f64 * 0.20).round();
    let y_40 = (y_max as f64 * 0.40).round();
    let y_60 = (y_max as f64 * 0.60).round();
    let y_80 = (y_max as f64 * 0.80).round();

    let final_class = if screen_size {
       "w-full md:w-3/5 flex flex-col gap-24 border p-8 border-teal-500 rounded-lg"
    } else {
       "w-full md:w-3/5 flex flex-col gap-48 border p-8 border-teal-500 rounded-lg"
    };
    

   
    render! {
        div {
            
            class: "flex flex-col md:flex-row gap-10 relative w-full",
            
            div{
                
                class: "w-full md:w-2/5 flex flex-col gap-20 border p-8 border-teal-500 rounded-lg", // Left side takes full width on small, 2/5 on larger screens",
                div {
                    class: "flex flex-col flex-1 pr-10",
                    h2 {
                        class: "text-lg md:text-2xl font-bold mb-20",
                        "Supply"
                    }
                    div {
                        class: "flex flex-col gap-8 my-auto",
                        OreValue {
                            title: "Circulating supply".to_string(),
                            detail: "The total amount of Spam that has been mined and claimed.".to_string(),
                            amount:  format!("{:.3}", circulating_supply) 
                        }
                        OreValue {
                            title: "Total supply".to_string(),
                            detail: "The total amount of Spam that has ever been mined.".to_string(),
                            amount: format!("{:.3}", spam_supply)
                        }
                    }
                }
                // 파이 차트 이미지 표시
                div {
                    class: "flex justify-center items-center pb-12", 
                    div {
                        class: "w-56 h-56 flex justify-center items-center rounded-full",
                        style: "background: conic-gradient(white {pie}%, #14b8a6 {pie}%)",
                        
                        div {
                            class: "absolute text-gray-800 font-bold text-center",
                            style: "transform: translateY(2.5rem);", // Adjust as needed to position the text
                            p {
                                "Unclaimed Spam"
                            }
                            p {
                                class: "mt-1", // Add margin-top for spacing
                                "{remaining_spam_text}"
                            }   
                           
                        }
                    }
                }  
        }
                
            // Right section: Transaction count chart
        div {
            // Conditionally apply gap-48 or gap-24 based on whether the alert banner is shown
            class:final_class, // Right side takes full width on small, 3/5 on larger screens",
            // Upper section with total transaction and dropdown
            div {
                class: "flex justify-between  h-1/10",
                h2 {
                    class: "text-lg md:text-2xl font-bold mb-8",
                    "Total Transaction: "
                    span {
                        class: "text-teal-500",
                        format!("{}", *count_sum.get())
                    }
                } 
               // console::log_1(&format!("max {}", max).into());
               div {
                class: "relative inline-block self-end",
                button {
                    class: if *show_dropdown.get() {
                        "mr-0 md:mr-12 mb-2 dropdown-button border text-center w-20 border-teal-500 rounded-lg"
                    } else {
                        "mr-0 md:mr-12 mb-2 dropdown-button border text-center w-20 border-teal-500 rounded-lg text-black dark:text-white hover:bg-teal-500 hover:text-white"
                    },
                    style: if *show_dropdown.get() {
                        "background-color: rgba(162, 216, 243, 0.75); color: white;"
                    } else {
                        ""
                    },
                    onclick: move |_| {
                        show_dropdown.set(!*show_dropdown.get());
                    },
                    match *selected_option.get() {
                        "hourly" => "12H",
                        "daily" => "7D",
                        "daily_30" => "30D",
                        "weekly" => "Weekly",
                        "monthly" => "Monthly",
                        _ => "12H",
                    }
                }
                if *show_dropdown.get() {
                    render!{
                        div {
                            class: "dropdown-content w-20 text-center absolute bg-teal-500 text-white border border-teal-500 rounded mt-2",
                            style: "background-color: rgba(162, 216, 243, 0.75); color: white;", 
                            button {
                                class: "block w-full px-4 py-2 text-sm hover:text-black", // Text turns black on hover
                                onclick: move |_| {
                                    selected_option.set("hourly");
                                    show_dropdown.set(false);
                                },
                                "12H"
                            }
                            button {
                                class: "block w-full px-4 py-2 text-sm hover:text-black", // Text turns black on hover
                                onclick: move |_| {
                                    selected_option.set("daily");
                                    show_dropdown.set(false);
                                },
                                "7D"
                            }
                            // button {
                            //     class: "hidden md:block w-full px-4 py-2 text-sm hover:text-black", // Text turns black on hover
                            //     onclick: move |_| {
                            //         selected_option.set("daily_30");
                            //         show_dropdown.set(false);
                            //     },
                            //     "30D"
                            // }
                            // button {
                            //     class: "block w-full px-4 py-2 text-sm hover:text-black", // Text turns black on hover
                            //     onclick: move |_| {
                            //         selected_option.set("weekly");
                            //         show_dropdown.set(false);
                            //     },
                            //     "Weekly"
                            // }      
                            // button {
                                //     class: "block w-full px-4 py-2 text-sm ",
                                //     onclick: move |_| {
                                //         selected_option.set("monthly");
                                //         show_dropdown.set(false);
                                //     },
                                //     "Monthly"
                                // }
                            }
                        }
                        
                    }
                    
                }
            }
            if recent_low_tx_count {
                rsx!(
                    div {
                        class: "bg-red-500 text-white p-2 rounded-lg text-center animate-pulse",  // Added banner with minimal margins
                            "Warning: Solana testnet is currently facing issues (Transactions may not be processing)."
                    }
                )
            }
            div {
                class: "relative flex w-full flex-col",
              
                div {
                    class: "absolute top-[100%] text-xs text-gray-500 pr-4",
                    p {"(UTC)"}
                }
                div {
                    class: "absolute top-[76%] text-xs text-gray-500 mr-2",
                    p { format!("{:.0}", y_20) }
                }
                div {
                    class: "absolute top-[56%] text-xs text-gray-500 mr-2",
                    p { format!("{:.0}", y_40) }
                }
                div {
                    class: "absolute top-[36%] text-xs text-gray-500 mr-2",
                    p { format!("{:.0}", y_60) }
                }
                div {
                    class: "absolute top-[16%] text-xs text-gray-500 mr-2",
                    p { format!("{:.0}", y_80) }
                }
               

                ul {
                    class: "chart h-auto dark:chart_dark pl-4",
                    for tx in page_data.iter() {
                        // Use conditional logic to truncate or show full count
                       
                        li {
                            span {
                                
                                // Use screen size-based conditional formatting (Tailwind utility classes)
                                style:"height: {tx.height}%; --bar-height: {tx.height}%;",
                                title: "{tx.timestamp}",
                                  "data-count":  "{tx.count}"  // Display truncated or full timestamp
                            }
                        }
                    }
                }
            
            }
            
        }

        }
    }
}

fn is_small_screen() -> bool {
    // You can use some logic or store screen size globally and return it here
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();

    width < 640.0 // Returns true for small screens (less than 640px)
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
                SpamIcon {
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
pub fn QuerySpamBalance(cx: Scope) -> Element {
    let address = use_state(cx, || "".to_string());
    let spam_balance = use_state(cx, || None);
    let claimable_spam_balance = use_state(cx, || None);
    let loading = use_state(cx, || false);
    let pubkey = use_state(cx, || "".to_string());
    let gateway = use_gateway(cx);

    use_future(cx, pubkey, |pubkey| {
        let gateway = gateway.clone();
        let claimable_spam_balance = claimable_spam_balance.clone();
        let spam_balance = spam_balance.clone();
        let loading = loading.clone();

        async move {
            loading.set(true);
            let pubkey_result = Pubkey::from_str(&pubkey);
            if let Ok(authority) = pubkey_result {
                let token_account_address = ore_token_account_address(authority);
                let proof_pubkey = proof_pubkey(authority);
                if let Ok(data) = gateway.rpc.get_account_data(&proof_pubkey).await {
                    if let Ok(p) = Proof::try_from_bytes(data.as_ref()) {
                        let claimable = (p.claimable_rewards as f64) / 10f64.powf(ore::TOKEN_DECIMALS as f64);
                        claimable_spam_balance.set(Some(claimable));
                    }
                }
                match gateway
                .rpc
                .get_token_account_balance(&token_account_address)
                .await
                {
                    Ok(token_account_balance) => {
                        spam_balance.set(Some(token_account_balance.ui_amount_string))
                    }
                    Err(err) => {
                        let err = GatewayError::from(err);
                        match err {
                            GatewayError::AccountNotFound => {
                                spam_balance.set(None)
                            }
                            _ => {
                                spam_balance.set(None)
                            }
                        }
                    }
                }                
                
            }
            loading.set(false);
        }
    });

    render! {
        div {
            class: "flex flex-col gap-6 border p-8 border-teal-500 rounded-lg",
            h2 {
                class: "text-lg md:text-2xl font-bold mb-2 lg:mb-4",
                "Query Spam Balance"
            }
            input {
                class: "p-4 border border-gray-300 rounded-lg text-black w-full",
                r#type: "text",
                placeholder: "🔍  Enter Solana Address",
                value: "{address}",
                oninput: move |evt| address.set(evt.value.clone()),
                onfocus: move |_| address.set("".to_string()),
            }
            button {
                class: "mt-4 bg-teal-500 hover:bg-teal-700 text-white font-bold py-2 px-4 rounded-lg",
                onclick: move |_| {
                    pubkey.set(address.get().to_string());
                },
                "Query"
            }
            if *loading.get() {
                rsx!(p { "Loading..." })

            } else {
                rsx! {
                    spam_balance.as_ref().map(|balance| {
                        let balance_text = format!("Spam Balance: {}", balance);
                        rsx!(p {
                            class: "mt-4 text-lg text-black dark:text-white",
                            "{balance_text}"
                        })
                    })
                    claimable_spam_balance.get().map(|claimable_balance| {
                        let claimable_balance_text = format!("Claimable Spam Balance: {}", claimable_balance);
                        rsx!(p {
                            class: "mt-4 text-lg text-black dark:text-white",
                            "{claimable_balance_text}"
                        })
                    })
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
            class: "flex flex-col md:flex-row gap-12 relative border p-8 border-teal-500 rounded-lg",  // Added border, padding, and rounded corners
            div {
                class: "flex flex-col flex-1 pr-0 md:pr-10",
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
            }
            div {
                class: "flex justify-end items-end",  // Align the image at the right side and bottom for all screens
                img {
                    src: "{solo}",  // Adjust the image path accordingly
                    class: "w-24 h-24 sm:w-32 sm:h-32 md:w-64 md:h-64",
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
        "Spam Treasury".to_string()
    } else {
        // Show only the first 6 characters of the owner account
        let mut account_display = token_account.owner.clone();
        if account_display.len() > 6 {
            if is_small_screen() {
                account_display = format!("{}...", &account_display[..8]);
            }    
        }
        account_display
    };
     // Parse the amount to a float and format to 2 decimal places
     let amount = token_account.token_amount.ui_amount_string.parse::<f64>()
    .map(|val| {
        if is_small_screen() {
            format!("{:.2}", val) // Format to 2 decimal places for small screens
        } else {
            format!("{:.3}", val) // Format to 3 decimal places for larger screens
        }
    })
    .unwrap_or_else(|_| token_account.token_amount.ui_amount_string.clone());

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
                SpamIcon {
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

