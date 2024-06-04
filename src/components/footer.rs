use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::components::{GithubIcon, XIcon};

#[component]
pub fn Footer(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row bg-teal-500 text-white w-full py-6 sm:py-10 px-4 sm:px-8 justify-between",
            p {
                class: "text-2xl font-semibold my-auto",  // Added margin to vertically center the text
                "SPAM"
            }
            div {
                class: "flex flex-row gap-8",
                Link {
                    to: "https://github.com/cookspam",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    GithubIcon {
                        class: "w-6 h-6 m-auto"
                    }
                }
                Link {
                    to: "https://x.com/CookHatChad",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    XIcon {
                        class: "w-5 h-5 m-auto"
                    }
                }
            }
        }
    }
}
