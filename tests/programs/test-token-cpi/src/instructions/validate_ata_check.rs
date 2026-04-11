use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct ValidateAtaCheck {
    #[account(associated_token::mint = mint, associated_token::authority = wallet)]
    pub ata: Account<Token>,
    pub mint: Account<Mint>,
    pub wallet: Signer,
    pub token_program: Program<Token>,
}

impl ValidateAtaCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
