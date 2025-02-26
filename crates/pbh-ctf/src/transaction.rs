use std::ops::{Deref, DerefMut};

use alloy_consensus::{TxEnvelope, TypedTransaction};
use alloy_network::{EthereumWallet, TransactionBuilder};
use alloy_primitives::{Address, Bytes, TxKind, U256};
use alloy_rpc_types_eth::{AccessList, TransactionInput, TransactionRequest};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::{SolCall, SolValue};
use base64::prelude::*;
use bon::builder;
use eyre::Result;
use semaphore_rs::{Field, hash_to_field, identity::Identity};
use serde::{Deserialize, Serialize};
use world_chain_builder_pbh::{
    date_marker::DateMarker,
    external_nullifier::{EncodedExternalNullifier, ExternalNullifier},
    payload::PBHPayload,
};
use world_chain_builder_test_utils::{
    WC_SEPOLIA_CHAIN_ID,
    bindings::{
        IMulticall3::{self, Call3},
        IPBHEntryPoint,
    },
};

use crate::{PBH_CTF_CONTRACT, PBHProof, world_id::WorldID};

#[derive(Debug, Clone, Default)]
pub struct CTFTransactionBuilder {
    pub tx: TransactionRequest,
}

impl CTFTransactionBuilder {
    pub fn new() -> Self {
        let tx = TransactionRequest::default()
            .gas_limit(130000)
            .max_fee_per_gas(1e8 as u128)
            .max_priority_fee_per_gas(1e8 as u128)
            .with_chain_id(WC_SEPOLIA_CHAIN_ID);

        CTFTransactionBuilder { tx }
    }

    pub async fn with_pbh_multicall(
        self,
        world_id: &WorldID,
        pbh_nonce: u16,
        calls: Vec<Call3>,
    ) -> Result<Self> {
        // Get the inclusion proof for the identity in the from the World Tree
        let from = Address::ZERO;
        let signal_hash = hash_to_field(&SolValue::abi_encode_packed(&(from, calls.clone())));
        let pbh_payload = world_id.pbh_payload(pbh_nonce, signal_hash).await?;

        let calldata = IPBHEntryPoint::pbhMulticallCall {
            calls,
            payload: pbh_payload.into(),
        };

        let tx = self
            .tx
            .input(TransactionInput::new(calldata.abi_encode().into()));
        Ok(Self { tx })
    }

    pub async fn build(self, signer: PrivateKeySigner) -> Result<TxEnvelope> {
        Ok(self.tx.build::<EthereumWallet>(&signer.into()).await?)
    }

    /// Sets the gas limit for the transaction.
    pub fn gas_limit(self, gas_limit: u64) -> Self {
        let tx = self.tx.gas_limit(gas_limit);
        Self { tx }
    }

    /// Sets the nonce for the transaction.
    pub fn nonce(self, nonce: u64) -> Self {
        let tx = self.tx.nonce(nonce);
        Self { tx }
    }

    /// Sets the maximum fee per gas for the transaction.
    pub fn max_fee_per_gas(self, max_fee_per_gas: u128) -> Self {
        let tx = self.tx.max_fee_per_gas(max_fee_per_gas);
        Self { tx }
    }

    /// Sets the maximum priority fee per gas for the transaction.
    pub fn max_priority_fee_per_gas(self, max_priority_fee_per_gas: u128) -> Self {
        let tx = self.tx.max_priority_fee_per_gas(max_priority_fee_per_gas);
        Self { tx }
    }

    /// Sets the recipient address for the transaction.
    pub fn to(self, to: Address) -> Self {
        let tx = self.tx.to(to);
        Self { tx }
    }

    /// Sets the value (amount) for the transaction.
    pub fn value(self, value: U256) -> Self {
        let tx = self.tx.value(value);
        Self { tx }
    }

    /// Sets the access list for the transaction.
    pub fn access_list(self, access_list: AccessList) -> Self {
        let tx = self.tx.access_list(access_list);
        Self { tx }
    }

    /// Sets the input data for the transaction.
    pub fn input(self, input: TransactionInput) -> Self {
        let tx = self.tx.input(input);
        Self { tx }
    }
}

/// Generates calldata for the PBH King of the Hill game, where the provided player
/// earns a point if they successfully capture the flag.
pub fn king_of_the_hill_calldata(player: Address) -> Bytes {
    crate::bindings::IPBHKotH::ctfCall { receiver: player }
        .abi_encode()
        .into()
}

/// Generates a multicall call, targeting the PBH King of the hill contract.
/// The provided player will earn a point if they successfully capture the flag.
pub fn king_of_the_hill_multicall(player: Address) -> Vec<Call3> {
    let call = IMulticall3::Call3 {
        target: PBH_CTF_CONTRACT,
        callData: crate::bindings::IPBHKotH::ctfCall { receiver: player }
            .abi_encode()
            .into(),
        allowFailure: false,
    };

    vec![call]
}
