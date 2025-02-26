//! This crate provides helpers for Generating provable transactions on World Chain.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use alloy_primitives::{Address, address};

pub mod bindings;
pub mod helpers;

/// The chain ID for WC Sepolia
pub const CHAIN_ID: u64 = 4801;

/// The PBH CTF contract address.
/// TODO: Update with deployed address
pub const PBH_CTF_CONTRACT: Address = address!("0E39accF2f9552a51a85A0A3412721AA597D1673");

/// The entrypoint contract for all PBH transactions.
pub const PBH_ENTRY_POINT: Address = address!("6e37bAB9d23bd8Bdb42b773C58ae43C6De43A590");

/// The Signature Aggregator for 4337 priority bundles.
pub const PBH_SIGNATURE_AGGREGATOR: Address = address!("ED5dc9CDB270818dCec0784bBdc8094082f0eBcB");

/// The URL for the inclusion proof endpoint.
pub const INCLUSION_PROOF_URL: &str = "https://signup-orb-ethereum.stage-crypto.worldcoin.dev";

// Re-exports
pub use helpers::*;

pub use world_chain_builder_pbh::{
    date_marker::DateMarker,
    external_nullifier::{EncodedExternalNullifier, ExternalNullifier},
    payload::Proof as PBHProof,
};

pub use world_chain_builder_test_utils::bindings::*;

pub use semaphore_rs::{
    Field, hash_to_field,
    identity::Identity,
    poseidon_tree::Proof,
    protocol::{generate_nullifier_hash, generate_proof},
};
