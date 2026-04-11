use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct ValidateTokenCheck {
    #[account(token::mint = mint, token::authority = authority)]
    pub token_account: Account<Token>,
    pub mint: Account<Mint>,
    pub authority: Signer,
    pub token_program: Program<Token>,
}

impl ValidateTokenCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
