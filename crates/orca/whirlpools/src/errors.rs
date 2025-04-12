use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorCode {
    #[error("Liquidity overflow")]
    LiquidityOverflow,

    #[error("Liquidity underflow")]
    LiquidityUnderflow,

    #[error("Liquidity net error")]
    LiquidityNetError,

    #[error("Divide by zero")]
    DivideByZero,

    #[error("MulDiv overflow")]
    MulDivOverflow,

    #[error("Multiplication shift right overflow")]
    MultiplicationShiftRightOverflow,

    #[error("Multiplication overflow")]
    MultiplicationOverflow,

    #[error("Number downcast error")]
    NumberDownCastError,

    #[error("Math overflow")]
    MathOverflow,

    #[error("Sqrt price x64 overflow")]
    SqrtPriceX64Overflow,

    #[error("Tick upper overflow")]
    TickUpperOverflow,

    #[error("Max token overflow")]
    MaxTokenOverflow,

    #[error("Liquidity add value error")]
    LiquidityAddValue,

    #[error("Liquidity sub value error")]
    LiquiditySubValue,

    #[error("Token max exceeded")]
    TokenMaxExceeded,

    #[error("Token min subceeded")]
    TokenMinSubceeded,

    #[error("Sqrt price out of bounds")]
    SqrtPriceOutOfBounds,
}

pub type Result<T> = std::result::Result<T, ErrorCode>;
