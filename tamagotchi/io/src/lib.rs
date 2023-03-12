#![no_std]

use parity_scale_codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::prelude::*;
use scale_info::TypeInfo;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<String, ()>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Tamagotchi;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}
