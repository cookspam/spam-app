#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
	components::CodeBlock,
	utils::asset_path,
};

#[component]
pub fn SpamDoc(cx: Scope) -> Element {
    let selected_tab = use_state(cx, || "SPAM 101");
	let spam_img = asset_path("spam_character1.png");

    let render_tab_content = |tab_name: &str| {
        match tab_name {
            "SPAM 101" => rsx!(
                div {
					div {
						class: "h-16", // Adds a 5rem (80px) space above the element
					}
					p {
						class: "text-xl sm:text-3xl md:text-4xl lg:text-5xl font-bold mb-8 font-hero",
						"Introducing SPAM: Revolutionizing Stress Testing on Solana"
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"Stress testing on Solana is essential for evaluating and enhancing blockchain performance and stability. However, inefficient methods hinder accurate performance assessment. We're here to change that."
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"Solana is one of the fastest blockchains, continuously improving with hardware (thanks to Moore's Law) and software optimizations. Yet, testing its limits remains a challenge."
					}
					h3 {
						class: "text-xl sm:text-2xl md:text-3xl font-semibold mb-8 mt-8",
						"The Challenges in Stress Testing"
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"Effective stress testing requires substantial computing and network resources. Current methods are costly, complex, and limited to a few organizations, creating a significant barrier to evaluating Solana's maximum performance."
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"Solana's testnet is used to test validator performance. Before any validator client upgrades or new Solana clients are deployed on the mainnet, they are first validated on the testnet. SPAM contributes to this process by stress-testing the validators under conditions similar to the mainnet, helping to validate and improve the performance of the validator clients."
					}
					h3 {
						class: "text-xl sm:text-2xl md:text-3xl font-semibold mb-8 mt-8",
						"The SPAM Solution"
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"Introducing SPAM: an efficient and cost-effective solution for stress testing Solana. It's simpler and more economical compared to traditional methods, providing a competitive edge."
					}
					h3 {
						class: "text-xl sm:text-2xl md:text-3xl font-semibold mb-8 mt-8",
						"Key Features of SPAM"
					}
					ul {
						class: "list-disc list-inside pl-4 text-lg",
						li { "Simpler and more economical" }
						li { "User incentives to motivate voluntary participation" }
						li { "Continuous stress testing on Solana testnet" }
						li { "Monitors and optimizes network performance" }
					}
					h3 {
						class: "text-xl sm:text-2xl md:text-3xl font-semibold mb-8 mt-8",
						"Incentives for User Participation"
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"To encourage user participation, SPAM offers mainnet rewards. Inspired by PoW implementations with extremely low difficulty, our system enables efficient network spamming."
					}
					h3 {
						class: "text-xl sm:text-2xl md:text-3xl font-semibold mb-8 mt-8",
						"Conclusion and Future Prospects"
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"SPAM is revolutionizing how we stress test blockchains like Solana. Stay tuned for more updates on how we're optimizing network performance and stability. Join us in this innovative journey."
					}
					
				}
				
            ),
			"TOKENOMICS" => rsx!(
				div {
					div {
						class: "h-16", // Adds a 5rem (80px) space above the element
					}
					p {
						class: "text-xl sm:text-3xl md:text-4xl lg:text-5xl font-bold mb-8 font-hero",
						"Spam Tokenomics Overview"
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"The SPAM tokenomics are designed to incentivize participation in Solana's stress-testing process while ensuring long-term sustainability and fairness. Below is an overview of the tokenomics for both the testnet and mainnet."
					}
					h3 {
						class: "text-xl sm:text-2xl md:text-3xl font-semibold mb-8 mt-8",
						"Spam Testnet Tokenomics"
					}
					ul {
						class: "list-disc list-inside pl-4 text-lg",
						li { "Follows the same tokenomics model as ORE V1" }
						li { "1 SPAM generated per minute" }
						li { "No maximum cap on SPAM tokens" }
						li { "21 million tokens expected to be generated over ~40 years" }
					}
					h3 {
						class: "text-xl sm:text-2xl md:text-3xl font-semibold mb-8 mt-8",
						"Spam Mainnet Tokenomics"
					}
					p {
						class: "text-lg leading-relaxed mb-8",
						"The mainnet tokenomics for SPAM are designed to reward early adopters and contributors. Testnet SPAM holders will receive airdrops on the mainnet."
					}
					p {
						class: "text-teal-500 text-lg leading-relaxed mb-8 mt-8",
						"⚙️ Detailed information on Spam Mainnet Tokenomics is coming soon."
					}
				}
				
				
				
			),
           "FAQ" => rsx!(
			div {
				div {
					class: "h-16", // Adds a 5rem (80px) space above the FAQ section
				}
				p {
					class: "text-xl sm:text-3xl md:text-4xl lg:text-5xl font-bold mb-8 font-hero",
					"FAQ"
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8",
					
					p {
						class: "text-xl font-semibold mb-4 text-white",
						"Q: Is mining Spam free?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: Absolutely! Mining Spam is completely free. All you need is some Solana Testnet tokens to get started."
					}
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8 ",
					
					p {
						class: "text-xl font-semibold mb-4 text-white",
						"Q: Which RPC should I use?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: You can use RPC services from Solana Labs, QuickNode, or All That Node. These are reliable options to get you up and running smoothly."
					}
					ul {
						class: "list-disc pl-5 text-lg mt-4",
						li {
							a {
								class: "text-teal-500 hover:text-teal-700",
								href: "https://api.testnet.solana.com",
								"Solana Labs RPC"
							}
							span {
								class: "text-white",
								" (free)"
							}
						}
						li {
							a {
								class: "text-teal-500 hover:text-teal-700",
								href: "https://www.quicknode.com/chains/sol?utm_term=solana%20testnet%20rpc&utm_campaign=Chain+%7C+Solana&utm_source=google&utm_medium=cpc&hsa_acc=1365030395&hsa_cam=20511511424&hsa_grp=153600396192&hsa_ad=672314075736&hsa_src=g&hsa_tgt=kwd-2193147571670&hsa_kw=solana%20testnet%20rpc&hsa_mt=p&hsa_net=adwords&hsa_ver=3&gad_source=1&gclid=Cj0KCQjw2ou2BhCCARIsANAwM2GOaw1VaZU0cujDbKl-sBf8J5RQYCYQUldm3Pngq3mN022x-KI7vacaAqOXEALw_wcB",
								"QuickNode"
							}
							span {
								class: "text-white",
								" (free up to 5M per month)"
							}
						}
						li {
							a {
								class: "text-teal-500 hover:text-teal-700",
								href: "https://www.allthatnode.com/",
								"All That Node"
							}
							span {
								class: "text-white",
								" (free 50k per day)"
							}
						}
					}
					
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8",
					
					p {
						class: "text-xl font-semibold mb-4 text-white",
						"Q: Where can I get Solana Testnet tokens?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: You can get Solana Testnet tokens from a few places:"
					}
					ul {
						class: "list-disc pl-5 text-lg",
						li {
							a {
								class: "text-teal-500 hover:text-teal-700",
								href: "https://faucet.solana.com/",
								"The official faucet "
							}
							span {
								class: "text-white",
								"(though it can be a bit unreliable)"
							}
						}
						li {
							a {
								class: "text-teal-500 hover:text-teal-700",
								href: "https://faucet.quicknode.com/solana/testnet",
								"QuickNode"
							}
						}
						li {
							a {
								class: "text-teal-500 hover:text-teal-700",
								href: "https://solfaucet.com/",
								"Solfaucet.com"
							}
						}
					}
					
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8",
					
					p {
						class: "text-xl font-semibold mb-4",
						"Q: What kind of device can I mine on?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: You can mine on pretty much anything. We’ve got a variety of clients to suit different devices. Each client has its own set of features, and we’ve got you covered whether you’re on desktop or mobile (mobile app coming soon)."
					}
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8",
					
					p {
						class: "text-xl font-semibold mb-4",
						"Q: Do I need a lot of network bandwidth?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: Not really. A 1Mbps connection is enough. But if you’re planning on mining all day long, we recommend using Wi-Fi or a wired connection for the best experience."
					}
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8",
					
					p {
						class: "text-xl font-semibold mb-4",
						"Q: Can I make money from this?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: You bet! We will reward you with mainnet tokens (to put it simply). We're also working on some revenue models, including LST (spamSOL), a DApp test marketplace, Solana client bug bounty programs, and even an $ORE mining pool. Stay tuned."
					}
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8",
					
					p {
						class: "text-xl font-semibold mb-4",
						"Q: How does Spam improve Solana if it's only on the testnet?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: Great question. Solana's testnet is used to test validator performance. Before any validator client upgrades or new Solana clients are deployed on the mainnet, they are first validated on the testnet. Spam plays a crucial role in this process by stress-testing the validators under conditions similar to the mainnet, helping to validate and improve the performance of the validator clients."
					}
				}
				
				div {
					class: "bg-gray-300 p-6 rounded-lg shadow-md mb-8",
					
					p {
						class: "text-xl font-semibold mb-4",
						"Q: Wen mainnet?"
					}
					p {
						class: "text-lg leading-relaxed",
						"A: We need more users to reach the mainnet. Spread the word, please! The more people join, the faster we can get there. We want you."
					}
				}
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
                    style: "{tab_style(&selected_tab, \"SPAM 101\")}",
                    onclick: move |_| selected_tab.set("SPAM 101"),
                    "SPAM 101"
                }
                button {
                    style: "{tab_style(&selected_tab, \"TOKENOMICS\")}",
                    onclick: move |_| selected_tab.set("TOKENOMICS"),
                    "TOKENOMICS"
                }
				button {
                    style: "{tab_style(&selected_tab, \"FAQ\")}",
                    onclick: move |_| selected_tab.set("FAQ"),
                    "FAQ"
                }
                // Add more buttons here for other apps
            }

            div {
                class: "pt-4",
                render_tab_content(&selected_tab)
            }
        }
    }
}

fn tab_style(selected_tab: &str, tab_name: &str) -> String {
    if selected_tab == tab_name {
        "background-color: #38b2ac; color: white; padding: 16px; cursor: pointer; font-weight: bold; transition: background-color 0.2s; border-radius: 10px 10px 0 0;".to_string()
    } else {
        "background-color: #2d3748; color: white; padding: 16px; cursor: pointer; font-weight: bold; transition: background-color 0.2s; border-radius: 10px 10px 0 0;".to_string()
    }
}
