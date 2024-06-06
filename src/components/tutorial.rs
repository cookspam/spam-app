use dioxus::prelude::*;

use crate::utils::asset_path;

#[component]
pub fn Tutorial(cx: Scope) -> Element {
    let tool = asset_path("Tool1.png");
    render! {
        div {
            class: "absolute right-4 sm:right-8 bottom-20 px-2 py-2 animate-bounce text-teal-500 rounded flex items-center space-x-2", // Change background to border
            img {
                src: "{tool}",
                class: "w-32 h-36", // Larger image size and reduce padding
                alt: "Tool"
            }
            div {
                class: "flex flex-col mr-4", // Increase padding on the right side of the text
                p {
                    class: "font-bold text-lg", // Make text bigger
                    "Start"
                }
                p {
                    class: "font-bold text-lg", // Make text bigger
                    "Spamming"
                }
            }
        }
    }
}
