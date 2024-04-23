use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

/// Accounts Canister Intialization Arguments
#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub bitcoin_network: BitcoinNetwork,
}

impl Storable for InitArgs {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for InitArgs {
    fn default() -> Self {
        Self {
            bitcoin_network: BitcoinNetwork::Regtest,
        }
    }
}
