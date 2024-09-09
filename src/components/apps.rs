#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
	components::CodeBlock,
	utils::asset_path,
};

#[component]
pub fn Apps(cx: Scope) -> Element {
    let selected_tab = use_state(cx, || "CLI");
	let spam_img = asset_path("spam_character1.png");

    let render_tab_content = |tab_name: &str| {
        match tab_name {
            "CLI" => rsx!(
                div {
					div {
						class: "h-16", // This adds a 5rem (80px) space above the element
					}
					p {
						class: "text-xl sm:text-3xl md:text-4xl lg:text-5xl font-bold mb-8 font-hero px-6",
						style: "margin-bottom: 24px;", // Inline styles for line height and margin
						"CLI"
					}
					
					p {
						style: "line-height: 1.75; margin-top: 24px; margin-bottom: 20px;", // Adjust line height and margin-bottom
						"Use the Spam CLI to run a miner on any machine. "
						"To get started, ensure you have Rust and cargo installed. "
					}
					CodeBlock {
						text: "curl https://sh.rustup.rs -sSf | sh"
					}
					p {
						style: "line-height: 1.75;  margin-top: 24px; margin-bottom: 24px;", // Adjust line height and margin-bottom
						"Next, install the Solana CLI and create a Solana keypair if you haven't done so already. "
					}
					
					// Break lines for small screen using Tailwind CSS classes
					div {
						class: "whitespace-pre-wrap break-all", // Ensures long text breaks correctly
						CodeBlock {
							text: "sh -c \"$(curl -sSfL https://release.solana.com/v1.18.4/install)\"\nsolana-keygen new",
						}
					}
					p {
						style: "line-height: 1.75;  margin-top: 24px; margin-bottom: 20px;", // Adjust line height and margin-bottom
						"Now, install the Spam CLI. Clone the Spam CLI repository from GitHub"
					}
					div {
						class: "whitespace-pre-wrap break-all", // Ensures long text breaks correctly
						CodeBlock {
							text: "git clone https://github.com/cookspam/spam-cli.git \n
							cd spam-cli \n
							cargo build --release"
						}
					}	
					p {
						style: "line-height: 1.75;  margin-top: 24px; margin-bottom: 20px;", // Adjust line height and margin-bottom
						"The Spam CLI uses your default Solana CLI config and identity. "
						"Ensure you have enough SOL topped up on this account to pay for transaction fees. "
						"To begin mining, use the mine command."
					}
					CodeBlock {
						text: "cargo run --release -- --rpc $RPC_URL --keypair $KEY_PATH mine"
					}
					p {
						style: "line-height: 1.75;  margin-top: 24px; margin-bottom: 20px;", // Adjust line height and margin-bottom
						"To see your mining rewards, run in terminal"
					}
					CodeBlock {
						text: "cargo run --release -- --rpc $RPC_URL --keypair $KEY_PATH rewards"
					}
					p {
						style: "line-height: 1.75;  margin-top: 24px; margin-bottom: 20px;", // Adjust line height and margin-bottom
						"To claim your rewards, run in terminal"
					}
					CodeBlock {
						text: "cargo run --release -- --rpc $RPC_URL --keypair $KEY_PATH claim"
					}
					p {
						style: "line-height: 1.75;  margin-top: 24px; margin-bottom: 20px;", // Adjust line height and margin-bottom
						"To mine in detached mode, use nohup."
					}
					CodeBlock {
						text: "nohup spam mine > output.log 2>&1 &"
					}
				}
            ),
           "Chrome Extension" => rsx!(
				div {
					// Chrome extension content as before
				}
           ),
           _ => rsx!(p { "Select a tab to view more information." }),
        }
    };

    render! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            
            div {
                class: "flex gap-4 border-b-2",
                
                button {
                    style: "{tab_style(&selected_tab, \"CLI\")}",
                    onclick: move |_| selected_tab.set("CLI"),
                    "CLI"
                }
                button {
                    style: "{tab_style(&selected_tab, \"Chrome Extension\")}",
                    onclick: move |_| selected_tab.set("Chrome Extension"),
                    "Chrome Extension"
                }
                button {
                    style: "{tab_style(&selected_tab, \"Desktop App\")}",
                    onclick: move |_| selected_tab.set("Desktop App"),
                    "Desktop App"
                }
            }

            div {
                class: "pt-4",
                render_tab_content(&selected_tab)
            }
        }
    }
}fn tab_style(selected_tab: &str, tab_name: &str) -> String {
    if selected_tab == tab_name {
        "background-color: #38b2ac; color: white; padding: 16px; cursor: pointer; font-weight: bold; transition: background-color 0.2s; border-radius: 10px 10px 0 0;".to_string()
    } else {
        "background-color: #2d3748; color: white; padding: 16px; cursor: pointer; font-weight: bold; transition: background-color 0.2s; border-radius: 10px 10px 0 0;".to_string()
    }
}