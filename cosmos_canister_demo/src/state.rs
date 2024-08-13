use std::{cell::RefCell, collections::BTreeMap};

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct InitArgs {
    pub schnorr_canister_principal: Principal,
}

pub fn init(args: InitArgs) {
    let state = RouteState::from(args);
    replace_state(state);
}

thread_local! {
    static __STATE: RefCell<Option<RouteState>> = RefCell::default();
}

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct RouteState {
    pub schnorr_canister_principal: Principal,
}

impl From<InitArgs> for RouteState {
    fn from(args: InitArgs) -> Self {
        Self {
            schnorr_canister_principal: args.schnorr_canister_principal,
        }
    }
}

pub fn mutate_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut RouteState) -> R,
{
    __STATE.with(|s| f(s.borrow_mut().as_mut().expect("State not initialized!")))
}

pub fn read_state<F, R>(f: F) -> R
where
    F: FnOnce(&RouteState) -> R,
{
    __STATE.with(|s| f(s.borrow().as_ref().expect("State not initialized!")))
}

pub fn replace_state(state: RouteState) {
    __STATE.with(|s| {
        *s.borrow_mut() = Some(state);
    });
}

pub fn take_state<F, R>(f: F) -> R
where
    F: FnOnce(RouteState) -> R,
{
    __STATE.with(|s| f(s.take().expect("State not initialized!")))
}