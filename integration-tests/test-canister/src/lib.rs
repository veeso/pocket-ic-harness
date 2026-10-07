use std::cell::RefCell;

use candid::{CandidType, Deserialize, Principal};

thread_local! {
    static COUNTER: RefCell<u64> = const { RefCell::new(0) };
    static PEER: RefCell<Option<Principal>> = const { RefCell::new(None) };
}

/// Argument for the `set_count` update method.
#[derive(CandidType, Deserialize)]
pub struct SetCountArg {
    pub value: u64,
}

/// Stores the optional peer canister principal passed at installation.
#[ic_cdk::init]
fn init(peer: Option<Principal>) {
    PEER.with(|p| *p.borrow_mut() = peer);
}

/// Returns the peer canister principal passed at installation, if any.
#[ic_cdk::query]
fn get_peer() -> Option<Principal> {
    PEER.with(|p| *p.borrow())
}

/// Returns the current counter value.
#[ic_cdk::query]
fn get_count() -> u64 {
    COUNTER.with(|c| *c.borrow())
}

/// Increments the counter by one and returns the new value.
#[ic_cdk::update]
fn increment() -> u64 {
    COUNTER.with(|c| {
        let mut count = c.borrow_mut();
        *count += 1;
        *count
    })
}

/// Sets the counter to the given value.
#[ic_cdk::update]
fn set_count(arg: SetCountArg) -> u64 {
    COUNTER.with(|c| {
        let mut count = c.borrow_mut();
        *count = arg.value;
        *count
    })
}
