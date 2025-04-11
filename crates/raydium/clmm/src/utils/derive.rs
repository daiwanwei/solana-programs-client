use solana_program::pubkey::Pubkey;

use crate::{
    constants::{
        AMM_CONFIG_SEED, OBSERVATION_SEED, POOL_SEED, POOL_TICK_ARRAY_BITMAP_SEED, POOL_VAULT_SEED,
        POSITION_SEED, TICK_ARRAY_SEED,
    },
    math::tick,
    ID,
};

pub fn derive_amm_config_pubkey(index: u16, program_id: Option<Pubkey>) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(&[AMM_CONFIG_SEED.as_bytes(), &index.to_be_bytes()], &program_id)
}

pub fn derive_pool_state_pubkey(
    amm_config: Pubkey,
    token_mint_0: Pubkey,
    token_mint_1: Pubkey,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[POOL_SEED.as_bytes(), amm_config.as_ref(), token_mint_0.as_ref(), token_mint_1.as_ref()],
        &program_id,
    )
}

pub fn derive_pool_vault_pubkey(
    pool_state: Pubkey,
    token_mint: Pubkey,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[POOL_VAULT_SEED.as_bytes(), pool_state.as_ref(), token_mint.as_ref()],
        &program_id,
    )
}

pub fn derive_personal_position_pubkey(
    position_nft: Pubkey,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(&[POSITION_SEED.as_bytes(), position_nft.as_ref()], &program_id)
}

pub fn derive_protocol_position_pubkey(
    pool_state: Pubkey,
    tick_lower_index: i32,
    tick_upper_index: i32,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[
            POSITION_SEED.as_bytes(),
            pool_state.as_ref(),
            tick_lower_index.to_be_bytes().as_ref(),
            tick_upper_index.to_be_bytes().as_ref(),
        ],
        &program_id,
    )
}

pub fn derive_tick_array_bitmap_pubkey(
    pool_state: Pubkey,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(), pool_state.as_ref()],
        &program_id,
    )
}

pub fn derive_tick_array_pubkey(
    pool_state: Pubkey,
    start_tick_index: i32,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[TICK_ARRAY_SEED.as_bytes(), pool_state.as_ref(), start_tick_index.to_be_bytes().as_ref()],
        &program_id,
    )
}

pub fn derive_tick_array_pubkey_by_tick_index(
    pool_state: Pubkey,
    tick_index: i32,
    tick_spacing: u16,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let start_tick_index = tick::get_array_start_index(tick_index, tick_spacing);
    derive_tick_array_pubkey(pool_state, start_tick_index, program_id)
}

pub fn derive_observation_pubkey(pool_state: Pubkey, program_id: Option<Pubkey>) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(&[OBSERVATION_SEED.as_bytes(), pool_state.as_ref()], &program_id)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_derive_pubkey() {
        let token_mint_0 = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let token_mint_1 =
            Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
        let nft_mint = Pubkey::from_str("8WBbAv9gS2S28vKsPTsVWajJW99PfYKk2Tw6wSdc8ysv").unwrap();

        let amm_config = Pubkey::from_str("9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x").unwrap();
        let pool_state = Pubkey::from_str("8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj").unwrap();
        let pool_vault_0 =
            Pubkey::from_str("6P4tvbzRY6Bh3MiWDHuLqyHywovsRwRpfskPvyeSoHsz").unwrap();
        let pool_vault_1 =
            Pubkey::from_str("6mK4Pxs6GhwnessH7CvPivqDYauiHZmAdbEFDpXFk9zt").unwrap();
        let personal_position =
            Pubkey::from_str("BYcjYTfkLLuAT7rKFr9SUTBsNy5ztotsC82Xae8buDS1").unwrap();
        let protocol_position =
            Pubkey::from_str("4f9KX1LAvcpRk5LrQDgXeWoYEygTWbfNK8R91jW7Gzoo").unwrap();
        let tick_array_bitmap =
            Pubkey::from_str("DoPuiZfJu7sypqwR4eiU7C5TMcmmiFoU4HaF5SoD8mRy").unwrap();
        let tick_array = Pubkey::from_str("HpdeMjnyao9w3o59UU81MhFT9vaWyFyGLRswYT55rpT7").unwrap();
        let observation = Pubkey::from_str("99jdk3KKYqqtBkZAE32GkgMhiR9csarMkT1WXKCkUhvV").unwrap();

        assert_eq!(derive_amm_config_pubkey(4, None).0, amm_config);
        assert_eq!(
            derive_pool_state_pubkey(amm_config, token_mint_0, token_mint_1, None).0,
            pool_state
        );
        assert_eq!(derive_pool_vault_pubkey(pool_state, token_mint_0, None).0, pool_vault_0);
        assert_eq!(derive_pool_vault_pubkey(pool_state, token_mint_1, None).0, pool_vault_1);
        assert_eq!(derive_personal_position_pubkey(nft_mint, None).0, personal_position);
        assert_eq!(derive_tick_array_bitmap_pubkey(pool_state, None).0, tick_array_bitmap);
        assert_eq!(derive_tick_array_pubkey(pool_state, -19860, None).0, tick_array);
        assert_eq!(
            derive_observation_pubkey(
                Pubkey::from_str("8uq2fhjohePKxb29fwN7cNBLHRKB2KAoHCCiPy5N3tqJ").unwrap(),
                None
            )
            .0,
            observation
        );
        assert_eq!(
            derive_protocol_position_pubkey(
                Pubkey::from_str("GTJ2S27UL7yZ3TdTwpKjfNcxeEZRkRPHjpj5Fubwb8Mk").unwrap(),
                41520,
                42180,
                None
            )
            .0,
            protocol_position
        );

        let amm_configs = vec![
            Pubkey::from_str("9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x").unwrap(),
            Pubkey::from_str("3XCQJQryqpDvvZBfGxR7CLAw5dpGJ9aa7kt1jRLdyxuZ").unwrap(),
            Pubkey::from_str("HfERMT5DRA6C1TAqecrJQFpmkf3wsWTMncqnj3RDg5aw").unwrap(),
            Pubkey::from_str("E64NGkDLLCdQ2yFNPcavaKptrEgmiQaNykUuLC1Qgwyp").unwrap(),
            Pubkey::from_str("A1BBtTYJd4i3xU8D6Tc2FzU6ZN4oXZWXKZnCxwbHXr8x").unwrap(),
        ];

        for amm_config in amm_configs {
            let (pool_state, _) =
                derive_pool_state_pubkey(amm_config, token_mint_0, token_mint_1, None);
            println!("pool_state: {}", pool_state);
        }
    }
}
