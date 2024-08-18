use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{
    Claim, Download, ExportKey, Home, ImportKey, Landing, Miner, Navbar, OreTokenomics, Apps,
    PageNotFound, Privacy, Send, Settings, SimpleNavbar, Stats, SpamDoc, Tx, User, WhatIsMining,
};


#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Landing {},

    #[layout(SimpleNavbar)]
        #[route("/what-is-mining")]
        WhatIsMining {},
        #[route("/ore-tokenomics")]
        OreTokenomics {},
        #[route("/download")]
        Download {},
        // #[route("/privacy")]
        // Privacy {},
        // #[route("/spam-doc")]
        // SpamDoc {},
       
    #[end_layout]

    #[layout(Navbar)]
        #[layout(Miner)]
           
            #[route("/mine")]
            Home {},
            #[route("/claim")]
            Claim {},
            #[route("/stats")]
            Stats {},
            #[route("/apps")]
            Apps {},
            #[route("/settings")]
            Settings {},
            #[route("/privacy")]
            Privacy {},
            #[route("/spam-doc")]
            SpamDoc {},
            #[route("/settings/export-key")]
            ExportKey {},
            #[route("/settings/import-key")]
            ImportKey {},
            #[route("/send/:to")]
            Send {
                to: String
            },
            #[route("/tx/:sig")]
            Tx {
                sig: String,
            },
            #[route("/u/:id")]
            User {
                id: String,
            },
        #[end_layout]
    #[end_layout]

    #[route("/:.._route")]
    PageNotFound { _route: Vec<String> }
}
