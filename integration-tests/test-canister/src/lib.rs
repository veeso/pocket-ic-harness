use std::cell::RefCell;

use candid::{CandidType, Deserialize};

thread_local! {
    static COUNTER: RefCell<u64> = const { RefCell::new(0) };
}

/// Argument for the `set_count` update method.
#[derive(CandidType, Deserialize)]
pub struct SetCountArg {
    pub value: u64,
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
