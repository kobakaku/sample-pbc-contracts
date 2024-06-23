#![doc = include_str!("../README.md")]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::{context::ContractContext, zk::ZkState};
use read_write_state_derive::ReadWriteState;

// mod zk_compute;

// #[derive(ReadWriteState, Debug)]
// #[repr(C)]
// pub struct SecretVarMetadata {}

#[state]
pub struct State {
    pub result: Option<bool>,
}

#[init]
pub fn initialize(
    _ctx: ContractContext,
    //  _zk_state: ZkState<SecretVarMetadata>
) -> State {
    State { result: None }
}

#[action(shortname = 0x01)]
pub fn verify(
    _ctx: ContractContext,
    mut state: State,
    // _zk_state: ZkState<SecretVarMetadata>,
    vote: bool,
) -> State {
    state.result = Some(vote);
    state
}
