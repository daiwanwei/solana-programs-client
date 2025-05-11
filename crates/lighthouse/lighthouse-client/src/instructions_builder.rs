use lighthouse::{
    find_memory_pda,
    instructions::{AssertAccountDeltaBuilder, MemoryCloseBuilder, MemoryWriteBuilder},
    types::{AccountDeltaAssertion, LogLevel},
    ID,
};
use solana_sdk::pubkey::Pubkey;

use crate::{error::Result, types::AccountDeltaInstructions};

pub struct AccountDeltaInstructionsBuilder {
    pub payer: Pubkey,
    pub account: Pubkey,
    pub memory_id: Option<u8>,
    pub program_id: Option<Pubkey>,
    pub assertions: Vec<AccountDeltaAssertion>,
}

impl AccountDeltaInstructionsBuilder {
    pub fn new(payer: Pubkey, account: Pubkey) -> Self {
        Self { memory_id: None, payer, account, program_id: None, assertions: vec![] }
    }

    pub fn memory_id(mut self, memory_id: u8) -> Self {
        self.memory_id = Some(memory_id);
        self
    }

    pub fn program_id(mut self, program_id: Pubkey) -> Self {
        self.program_id = Some(program_id);
        self
    }
}

impl AccountDeltaInstructionsBuilder {
    pub fn add_assertion(mut self, assertion: AccountDeltaAssertion) -> Self {
        self.assertions.push(assertion);
        self
    }

    pub fn build(self) -> Result<AccountDeltaInstructions> {
        let memory_id = self.memory_id.unwrap_or(0);
        let program_id = self.program_id.unwrap_or(ID);
        let assertions = self.assertions;

        let (memory_pda, memory_bump) = find_memory_pda(self.payer, memory_id);

        let before_ix = vec![MemoryWriteBuilder::new()
            .payer(self.payer)
            .source_account(self.account)
            .program_id(program_id)
            .memory(memory_pda)
            .memory_id(memory_id)
            .instruction()];
        let mut after_ix = vec![];
        for assertion in assertions {
            after_ix.push(
                AssertAccountDeltaBuilder::new()
                    .account_a(memory_pda)
                    .account_b(self.account)
                    .assertion(assertion)
                    .log_level(LogLevel::PlaintextMessage)
                    .instruction(),
            );
        }

        let close_ix = MemoryCloseBuilder::new()
            .payer(self.payer)
            .program_id(program_id)
            .memory(memory_pda)
            .memory_bump(memory_bump)
            .memory_id(memory_id)
            .instruction();

        after_ix.push(close_ix);

        Ok(AccountDeltaInstructions { memory_id, account: self.account, before_ix, after_ix })
    }
}
