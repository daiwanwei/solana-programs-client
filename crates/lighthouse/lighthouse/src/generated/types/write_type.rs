//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>

use borsh::{BorshDeserialize, BorshSerialize};

use crate::generated::types::{AccountInfoField, ClockField, DataValue};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WriteType {
    AccountData { offset: u16, data_length: u16 },
    AccountInfoField(AccountInfoField),
    DataValue(DataValue),
    Clock(ClockField),
}
