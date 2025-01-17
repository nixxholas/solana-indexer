//! Miscellaneous utility functions.
#![allow(dead_code)]

use mpl_token_metadata::state::{MasterEditionV1, MasterEditionV2};
#[cfg(feature = "solana-program")]
use {
    indexer_core::prelude::*,
    mpl_token_metadata::state::Key,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult},
    solana_sdk::{account::Account, pubkey::Pubkey},
};

/// Borrow a `solana-sdk` account as a `solana-program` account info struct.
#[cfg(feature = "solana-program")]
pub fn account_as_info<'a>(
    key: &'a Pubkey,
    is_signer: bool,
    is_writable: bool,
    acct: &'a mut Account,
) -> AccountInfo<'a> {
    AccountInfo::new(
        key,
        is_signer,
        is_writable,
        &mut acct.lamports,
        &mut *acct.data,
        &acct.owner,
        acct.executable,
        acct.rent_epoch,
    )
}

/// Borrow an account's raw as a `solana-program` account info struct.
#[cfg(feature = "solana-program")]
pub fn account_data_as_info<'a>(
    key: &'a Pubkey,
    data: &'a mut [u8],
    owner: &'a Pubkey,
    lamports: &'a mut u64,
) -> AccountInfo<'a> {
    AccountInfo::new(key, false, false, lamports, data, owner, false, 0)
}

/// Convenience wrapper for Metaplex's [`MasterEdition`] trait and structs
#[derive(Debug)]
pub enum MasterEdition {
    /// A v1 master edition
    V1(MasterEditionV1),
    /// A v2 master edition
    V2(MasterEditionV2),
}

impl MasterEdition {
    /// Construct the correct master edition from an account
    ///
    /// # Errors
    /// This function fails if the account cannot be parsed as a v1 account or a v2 account.
    #[cfg(feature = "solana-program")]
    pub fn from_account_info(
        info: &AccountInfo,
    ) -> Result<Self, solana_sdk::program_error::ProgramError> {
        MasterEditionV2::from_account_info(info)
            .map(Self::V2)
            .or_else(|e| {
                debug!("Couldn't parse MasterEditionV2: {:?}", e);
                MasterEditionV1::from_account_info(info).map(Self::V1)
            })
    }
}

#[cfg(feature = "solana-program")]
impl mpl_token_metadata::state::MasterEdition for MasterEdition {
    fn key(&self) -> Key {
        match self {
            Self::V1(m) => m.key(),
            Self::V2(m) => m.key(),
        }
    }

    fn supply(&self) -> u64 {
        match self {
            Self::V1(m) => m.supply(),
            Self::V2(m) => m.supply(),
        }
    }

    fn set_supply(&mut self, supply: u64) {
        match self {
            Self::V1(m) => m.set_supply(supply),
            Self::V2(m) => m.set_supply(supply),
        }
    }

    fn max_supply(&self) -> Option<u64> {
        match self {
            Self::V1(m) => m.max_supply(),
            Self::V2(m) => m.max_supply(),
        }
    }

    fn save(&self, account: &AccountInfo) -> ProgramResult {
        match self {
            Self::V1(m) => m.save(account),
            Self::V2(m) => m.save(account),
        }
    }
}
