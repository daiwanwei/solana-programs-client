#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreviewSwapResult {
    pub amount_in: u64,
    pub amount_out: u64,
    pub threshold: u64,
    pub fee: u64,
}
