use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("The given name is too long")]
    NameTooLong,
    #[msg("The calculation number is too large or too small")]
    ArithematicOverflow,
}