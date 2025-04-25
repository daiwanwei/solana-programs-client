#[derive(Debug, Clone)]
pub struct DecodedAccount<T> {
    pub address: solana_program::pubkey::Pubkey,
    pub account: solana_sdk::account::Account,
    pub data: T,
}

#[derive(Debug, Clone)]
pub enum MaybeAccount<T> {
    Exists(DecodedAccount<T>),
    NotFound(solana_program::pubkey::Pubkey),
}
