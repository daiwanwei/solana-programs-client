use std::sync::{Arc, Mutex};

use litesvm::LiteSVM;
use program_test_utils::svm::update_clock;
use raydium_clmm::ID;
use raydium_clmm_test::program_test::{RaydiumClmmTest, RaydiumClmmTestBuilder};
use solana_sdk::signature::{Keypair, Signer};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_clmm() {
        let (clmm_test, svm, payer) = build_clmm_test();

        let (ata_0, _) = clmm_test.get_or_create_ata(clmm_test.mint_0, payer.pubkey()).unwrap();
        let (ata_1, _) = clmm_test.get_or_create_ata(clmm_test.mint_1, payer.pubkey()).unwrap();

        let _unused =
            clmm_test.mint_to(clmm_test.mint_0, ata_0, 1_000_000_000_000_000_000).unwrap();
        let _unused =
            clmm_test.mint_to(clmm_test.mint_1, ata_1, 1_000_000_000_000_000_000).unwrap();

        update_clock(&mut svm.lock().unwrap(), 1, 1000);

        let _unused = clmm_test
            .open_position_v2(
                -30,
                30,
                1000000000,
                1000000000000000000,
                1000000000000000000,
                None,
                None,
                Some(&payer),
            )
            .unwrap();

        let (position_nft_mint, _) = clmm_test
            .open_position_v2(
                -60,
                -30,
                1000000000,
                1000000000000000000,
                1000000000000000000,
                None,
                None,
                Some(&payer),
            )
            .unwrap();

        let _unused = clmm_test
            .increase_liquidity(
                position_nft_mint,
                1000,
                1000000000,
                1000000000,
                None,
                None,
                Some(&payer),
            )
            .unwrap();

        let _unused = clmm_test.swap_v2(10, 0, 0, true, true, None, None, Some(&payer)).unwrap();
    }
}

fn build_clmm_test() -> (RaydiumClmmTest, Arc<Mutex<LiteSVM>>, Arc<Keypair>) {
    let program_id = ID;
    let mut svm = LiteSVM::new().with_sigverify(false);
    svm.add_program(program_id, include_bytes!("fixtures/raydium_clmm.so"));
    let metadata_program_id =
        solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    svm.add_program(metadata_program_id, include_bytes!("fixtures/metaplex_metadata.so"));

    let payer = Arc::new(Keypair::new());

    let _unused = svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let svm = Arc::new(Mutex::new(svm));

    let clmm_test = RaydiumClmmTestBuilder::new(svm.clone()).admin(payer.clone()).build().unwrap();

    (clmm_test, svm, payer)
}
