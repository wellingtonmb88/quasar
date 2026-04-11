use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct ValidateMintCheck {
    #[account(mint::authority = mint_authority, mint::decimals = 6)]
    pub mint: Account<Mint>,
    pub mint_authority: Signer,
    pub token_program: Program<Token>,
}

impl ValidateMintCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
