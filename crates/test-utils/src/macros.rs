#[macro_export]
macro_rules! prepare_anchor_ix {
    ($program_id:expr, $ix:expr, $accounts:expr, $remaining_accounts:expr) => {{
        let mut builder = solana_instruction_builder::InstructionBuilder::new($program_id)
            .add_anchor_data($ix)
            .add_named_accounts_from_struct($accounts);

        if let Some(remaining_accounts) = $remaining_accounts {
            builder = builder.add_remaining_accounts(remaining_accounts);
        }

        builder.build()
    }};
}
