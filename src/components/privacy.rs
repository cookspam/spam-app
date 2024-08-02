use dioxus::prelude::*;

#[component]
pub fn Privacy(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            p {
                class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-8 font-hero",
                "Spam Chrome Extension Privacy Policy"
            }
            p {
               
               "This extension does not collect, store, or transmit any personal information from users. All operations and functionalities are executed locally on the user's device. "}
			p {   "Any data required for the extension's functionality, such as public and private keys or RPC URLs, is stored securely on the user's device and not accessed by or transmitted to any external servers. "}
			   
            p { "We are committed to ensuring the privacy and security of our users and do not engage in any data collection practices."}
            
            
        }
    }
}
