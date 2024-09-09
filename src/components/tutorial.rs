use dioxus::prelude::*;

use crate::utils::asset_path;

#[component]
pub fn Tutorial(cx: Scope) -> Element {
    let tool = asset_path("Tool1.png");
    render! {
        div {
            class: "absolute right-4 sm:right-8 bottom-20 px-2 py-2 animate-bounce text-teal-500 rounded flex items-center space-x-2", // No changes here
            img {
                src: "{tool}",
                class: "w-24 h-28 md:w-32 md:h-36", // Smaller image for small screens and original size for medium screens and up
                alt: "Tool"
            }
            div {
                class: "flex flex-col mr-2 md:mr-4", // Smaller right margin for small screens and larger for medium screens
                p {
                    class: "font-bold text-sm md:text-lg", // Smaller text for small screens, larger for medium screens
                    "Start"
                }
                p {
                    class: "font-bold text-sm md:text-lg", // Same adjustment for the second line of text
                    "Spamming"
                }
            }
        }
        
    }
}
