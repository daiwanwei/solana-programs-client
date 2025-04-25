#[macro_export]
macro_rules! sign_and_send_transaction {
    ($svm:expr, $instructions:expr, $signer:expr) => {{
        $crate::svm::sign_and_send_transaction($svm, $instructions, $signer, None).inspect_err(
            |e| {
                println!("Program logs: {:?}", e.meta.logs);
            },
        )
    }};
    ($svm:expr, $instructions:expr, $signer:expr, $signers:expr) => {{
        $crate::svm::sign_and_send_transaction($svm, $instructions, $signer, Some($signers))
            .inspect_err(|e| {
                println!("Program logs: {:?}", e.meta.logs);
            })
    }};
}
