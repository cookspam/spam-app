mod claim_confirm;
mod claim_done;
mod claim_edit;
mod claim_preview;

use dioxus::prelude::*;

use crate::{
    components::claim_modal::{
        claim_confirm::ClaimConfirm, claim_done::ClaimDone, claim_edit::ClaimEdit,
    },
    gateway::AsyncResult,
    hooks::use_proof,
};

pub enum ClaimStep {
    // Preview,
    Edit,
    Confirm,
    Done,
}

// #[derive(Props)]
// pub struct ClaimProps<'a> {
//     pub balance_handle: &'a UseFuture<()>,
// }

#[component]
pub fn Claim(cx: Scope) -> Element {
    // pub fn Claim<'a>(cx: Scope<'a, ClaimProps<'a>>) -> Element {
    let (proof_rw, proof_) = use_proof(cx);
    let proof = *proof_rw.read().unwrap();
    let claim_step = use_state(cx, || ClaimStep::Edit);
    let amount_input = use_state(cx, || "".to_string());
    // let balance_ = cx.props.balance_handle;

    let parsed_amount: u64 = match amount_input.get().parse::<f64>() {
        Ok(n) => (n * 10f64.powf(ore::TOKEN_DECIMALS.into())) as u64,
        Err(_) => 0,
    };

    let amount = match &proof {
        AsyncResult::Ok(proof) => proof.claimable_rewards,
        _ => 0,
    };

    match claim_step.get() {
        ClaimStep::Edit => {
            render! {
                ClaimEdit {
                    claim_step: claim_step,
                    amount_input: amount_input,
                    max_rewards: amount,
                    parsed_amount: parsed_amount,
                }
            }
        }
        ClaimStep::Confirm => {
            render! {
                ClaimConfirm {
                    claim_step: claim_step,
                    amount: parsed_amount,
                    // balance_handle: balance_,
                    proof_handle: proof_,
                }
            }
        }
        ClaimStep::Done => {
            render! {
                ClaimDone {}
            }
        }
    }
}
