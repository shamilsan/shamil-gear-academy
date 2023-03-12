#![no_std]

use gstd::{exec, msg, prelude::*};
use once_cell::sync::OnceCell;
use tamagotchi_io::*;

static TMG: OnceCell<Tamagotchi> = OnceCell::new();

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Unable to load message payload");
    TMG.set(Tamagotchi {
        name,
        date_of_birth: exec::block_timestamp(),
    })
    .expect("Unable to set");
}

#[no_mangle]
extern "C" fn handle() {}

#[no_mangle]
extern "C" fn state() {
    let tmg = TMG.get().expect("The contract is not initialized");
    msg::reply(tmg, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
