use anchor_trait::Discriminator;
use borsh::BorshDeserialize;
use solana_account::Account;
use solana_client_core::MaybeAccount;
use solana_clock::Slot;
use solana_deserialize::account::{decode_solana_account_by_borsh, decode_solana_account_by_pack};
use solana_program_pack::Pack;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    constants::MAX_FETCH_ACCOUNTS,
    error::{AccountError, Result},
};

pub async fn fetch_solana_account_by_borsh<T>(
    client: &RpcClient,
    pubkey: Pubkey,
    commitment_config: CommitmentConfig,
) -> Result<(MaybeAccount<T>, Slot)>
where
    T: BorshDeserialize,
{
    fetch_and_deserialize_account(client, pubkey, commitment_config, |data| {
        if let Ok(decoded_account) = decode_solana_account_by_borsh::<T>(&pubkey, data) {
            MaybeAccount::Exists(decoded_account)
        } else {
            MaybeAccount::NotFound(pubkey)
        }
    })
    .await
}

pub async fn fetch_solana_accounts_by_borsh<T>(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
) -> Result<Vec<(MaybeAccount<T>, Slot)>>
where
    T: BorshDeserialize + Discriminator,
{
    fetch_and_deserialize_accounts(client, pubkeys, commitment_config, |pubkey, data| {
        if let Ok(decoded_account) = decode_solana_account_by_borsh::<T>(pubkey, data) {
            MaybeAccount::Exists(decoded_account)
        } else {
            MaybeAccount::NotFound(*pubkey)
        }
    })
    .await
}

pub async fn fetch_solana_account_by_pack<T>(
    client: &RpcClient,
    pubkey: Pubkey,
    commitment_config: CommitmentConfig,
) -> Result<(MaybeAccount<T>, Slot)>
where
    T: Pack,
{
    fetch_and_deserialize_account(client, pubkey, commitment_config, |data| {
        if let Ok(decoded_account) = decode_solana_account_by_pack::<T>(&pubkey, data) {
            MaybeAccount::Exists(decoded_account)
        } else {
            MaybeAccount::NotFound(pubkey)
        }
    })
    .await
}

pub async fn fetch_solana_accounts_by_pack<T>(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
) -> Result<Vec<(MaybeAccount<T>, Slot)>>
where
    T: Pack,
{
    fetch_and_deserialize_accounts(client, pubkeys, commitment_config, |pubkey, data| {
        if let Ok(decoded_account) = decode_solana_account_by_pack::<T>(pubkey, data) {
            MaybeAccount::Exists(decoded_account)
        } else {
            MaybeAccount::NotFound(*pubkey)
        }
    })
    .await
}

pub async fn fetch_accounts(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
) -> Result<Vec<(Pubkey, Option<Account>, Slot)>> {
    if pubkeys.is_empty() {
        return Ok(Vec::new());
    }

    let mut accounts = Vec::with_capacity(pubkeys.len());

    for chunk in pubkeys.chunks(MAX_FETCH_ACCOUNTS) {
        let chunk_accounts = client
            .get_multiple_accounts_with_commitment(chunk, commitment_config)
            .await
            .map_err(|e| AccountError::FetchAccount { source: e })?;

        let slot = chunk_accounts.context.slot;

        let chunk_accounts = chunk_accounts
            .value
            .into_iter()
            .zip(chunk.iter())
            .map(|(account, pubkey)| (pubkey.clone(), account, slot))
            .collect::<Vec<_>>();
        accounts.extend(chunk_accounts);
    }

    Ok(accounts)
}

pub async fn fetch_and_deserialize_account<T>(
    client: &RpcClient,
    pubkey: Pubkey,
    commitment_config: CommitmentConfig,
    decode: impl Fn(&Account) -> MaybeAccount<T>,
) -> Result<(MaybeAccount<T>, Slot)> {
    let res = if let Ok(res) = client.get_account_with_commitment(&pubkey, commitment_config).await
    {
        res
    } else {
        return Ok((MaybeAccount::NotFound(pubkey), 0));
    };

    let slot = res.context.slot;
    let account = if let Some(account) = res.value {
        decode(&account)
    } else {
        MaybeAccount::NotFound(pubkey)
    };
    Ok((account, slot))
}

pub async fn fetch_and_deserialize_accounts<T>(
    client: &RpcClient,
    pubkeys: &[Pubkey],
    commitment_config: CommitmentConfig,
    decode: impl Fn(&Pubkey, &Account) -> MaybeAccount<T>,
) -> Result<Vec<(MaybeAccount<T>, Slot)>> {
    if pubkeys.is_empty() {
        return Ok(vec![]);
    }

    let accounts = fetch_accounts(client, pubkeys, commitment_config).await?;

    let accounts = accounts
        .into_iter()
        .map(|(pubkey, account, slot)| {
            if let Some(account) = account {
                (decode(&pubkey, &account), slot)
            } else {
                (MaybeAccount::NotFound(pubkey), slot)
            }
        })
        .collect::<Vec<_>>();

    Ok(accounts)
}
