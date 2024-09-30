use dioxus::prelude::*;
use dioxus_router::prelude::Link;

//use crate::components::{GithubIcon, XIcon};

use crate::{
	components::{GithubIcon, XIcon},
	utils::asset_path,
};


#[component]
pub fn Footer(cx: Scope, hidden: bool) -> Element {
    let spam_img = asset_path("spam_character1.png");
    let discord_img = asset_path("discord.png");
    let youtube_img = asset_path("youtube.png");
    if *hidden {
        return None;
    }
    render! {
        div {
            class: "flex flex-row bg-teal-500 text-white w-full py-6 sm:py-10 px-4 sm:px-8 justify-between",
            p {
                class: "text-2xl font-semibold my-auto",  // Added margin to vertically center the text
                "SPAM"
            }
            div {
                class: "flex flex-row gap-6",
                Link {
                    to: "https://x.com/CookHatChad",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    XIcon {
                        class: "w-5 h-5 m-auto"
                    }
                }
                Link {
                    to: "https://chromewebstore.google.com/detail/spam-chrome-extension/okchhmhmdibmceakkjehbfjipibmonbe",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    img {
                        src: "{spam_img}",  // Adjust the image path accordingly
                        class: "w-6 h-6  m-auto",
                        alt: "Spam Character"
                    }
                }
                Link {
                    to: "https://discord.gg/Kn9p4Syx",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    img {
                        src: "{discord_img}",  // Adjust the image path accordingly
                        class: "w-6 h-6 m-auto",
                        alt: "Discord"
                    }
                }
                Link {
                    to: "https://www.youtube.com/@spam_sol",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    img {
                        src: "{youtube_img}",  // Adjust the image path accordingly
                        class: "w-8 h-8  m-auto",
                        alt: "YouTube"
                    }
                }
                Link {
                    to: "https://github.com/cookspam",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    GithubIcon {
                        class: "w-6 h-6 m-auto"
                    }
                }
               
            }
        }
    }
}
