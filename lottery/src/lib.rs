#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::sorted_vec_map::SortedVecSet;
use std::time::{SystemTime, UNIX_EPOCH};

/// The state of the lottery, which is persisted on-chain.
#[state]
pub struct LotteryState {
    participants: SortedVecSet<Address>,
}

/// Initialize a new lottery.
///
/// # Arguments
///
/// * `_ctx` - the contract context containing information about the sender and the blockchain.
///
/// # Returns
///
/// The initial state of the lottery, with no participants.
///
#[init]
pub fn initialize(_ctx: ContractContext) -> LotteryState {
    LotteryState {
        participants: SortedVecSet::new(),
    }
}

/// Enter the lottery.
///
/// # Arguments
///
/// * `ctx` - the contract context containing information about the sender and the blockchain.
///
/// # Returns
///
/// The updated state of the lottery with the participant added.
///
#[action(shortname = 0x01)]
pub fn enter(ctx: ContractContext, mut state: LotteryState) -> LotteryState {
    state.participants.insert(ctx.sender);
    state
}

/// Pick the winner of the lottery.
///
/// # Arguments
///
/// * `ctx` - the contract context containing information about the sender and the blockchain.
///
/// # Returns
///
/// The winner of the lottery.
///
#[action(shortname = 0x02)]
pub fn pick_winner(ctx: ContractContext, mut state: LotteryState) -> Option<Address> {
    // Get the current system time
    let current_time = SystemTime::now();

    // Convert system time to UNIX timestamp
    let timestamp = match current_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => return None, // Handle error if system time is before UNIX epoch
    };

    // Use the timestamp to determine the winner
    if state.participants.len() > 0 {
        let index = (timestamp % state.participants.len() as u64) as usize;
        // Iterate over the participants to find the winner at the specified index
        let winner = state.participants.iter().nth(index).cloned();
        // Remove the winner from the set of participants
        if let Some(winner_address) = winner {
            state.participants.remove(&winner_address);
        }
        winner
    } else {
        None
    }
}