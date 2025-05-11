use lighthouse::{
    find_memory_pda,
    instructions::{
        AssertAccountDataBuilder, AssertAccountDeltaBuilder, MemoryCloseBuilder, MemoryWriteBuilder,
    },
    types::{
        AccountDeltaAssertion, DataValueAssertion, DataValueDeltaAssertion, IntegerOperator,
        LogLevel, WriteType,
    },
    ID,
};
use lighthouse_common::types::CompactU64;
use litesvm::LiteSVM;
use program_test_utils::{
    account::get_solana_account_by_pack,
    sign_and_send_transaction,
    token::{create_mint, get_or_create_ata, mint_to},
};
use solana_client_core::types::MaybeAccount;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token::{state::Account, ID as SPL_TOKEN_ID};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_account_assertion() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { mut svm, admin, user0, .. } = create_fixture()?;

        let amount = match get_solana_account_by_pack::<Account>(&svm, &user0.token_account) {
            MaybeAccount::Exists(account) => account.data.amount,
            MaybeAccount::NotFound(_) => {
                return Err("Account not found".into());
            }
        };

        let ix = AssertAccountDataBuilder::new()
            .target_account(user0.token_account)
            .log_level(LogLevel::PlaintextMessage)
            .offset(CompactU64::from(64u64))
            .assertion(DataValueAssertion::U64 { value: amount, operator: IntegerOperator::Equal })
            .instruction();

        let metadata = sign_and_send_transaction!(&mut svm, &[ix], &admin).unwrap();
        println!("metadata: {:?}", metadata);

        Ok(())
    }

    #[test]
    fn test_delta_assertion() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { mut svm, admin, user0, .. } = create_fixture()?;
        let memory_id = 0;
        let (memory_pda, memory_bump) = find_memory_pda(admin.pubkey(), memory_id);
        let write_ix = MemoryWriteBuilder::new()
            .payer(admin.pubkey())
            .source_account(user0.token_account)
            .program_id(ID)
            .memory(memory_pda)
            .memory_id(memory_id)
            .write_offset(0u8.into())
            .memory_bump(memory_bump)
            .write_type(WriteType::AccountData { offset: 0, data_length: 72 })
            .instruction();

        let assert_ix = AssertAccountDeltaBuilder::new()
            .account_a(memory_pda)
            .account_b(user0.token_account)
            .assertion(AccountDeltaAssertion::Data {
                a_offset: 64u8.into(),
                b_offset: 64u8.into(),
                assertion: DataValueDeltaAssertion::U64 {
                    value: 0,
                    operator: IntegerOperator::GreaterThanOrEqual,
                },
            })
            .log_level(LogLevel::PlaintextMessage)
            .instruction();

        let close_ix = MemoryCloseBuilder::new()
            .payer(admin.pubkey())
            .program_id(ID)
            .memory(memory_pda)
            .memory_bump(memory_bump)
            .memory_id(memory_id)
            .instruction();

        let metadata =
            sign_and_send_transaction!(&mut svm, &[write_ix, assert_ix, close_ix], &admin).unwrap();
        println!("metadata: {:?}", metadata);

        Ok(())
    }
}

fn create_fixture() -> Result<Fixture, Box<dyn std::error::Error>> {
    let program_id = ID;
    let mut svm = LiteSVM::new().with_sigverify(false);
    svm.add_program(program_id, include_bytes!("fixtures/lighthouse.so"));
    let admin = Keypair::new();

    let _unused = svm.airdrop(&admin.pubkey(), 1_000_000_000).unwrap();

    let (mint, _) = create_mint(&mut svm, &admin, &admin.pubkey(), 6, SPL_TOKEN_ID).unwrap();

    let user0 = create_user(&mut svm, &admin, &mint);
    let user1 = create_user(&mut svm, &admin, &mint);
    let user2 = create_user(&mut svm, &admin, &mint);

    Ok(Fixture { svm, admin, mint, user0, user1, user2 })
}

fn create_user(svm: &mut LiteSVM, admin: &Keypair, mint: &Pubkey) -> User {
    let user = Keypair::new();
    let _unused = svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();
    let (token_account, _) = get_or_create_ata(svm, admin, mint, &user.pubkey()).unwrap();

    let _unused =
        mint_to(svm, admin, mint, &token_account, &[admin], 1_000_000_000_000_000_000).unwrap();

    User { keypair: user, token_account }
}

#[allow(dead_code)]
struct Fixture {
    svm: LiteSVM,

    admin: Keypair,

    mint: Pubkey,

    user0: User,
    user1: User,
    user2: User,
}

#[allow(dead_code)]
struct User {
    keypair: Keypair,
    token_account: Pubkey,
}
