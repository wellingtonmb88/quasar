use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

/// No `token_program` field — program is known at compile time from
/// Account<Token>.
#[derive(Accounts)]
pub struct ValidateTokenNoProgram {
    #[account(token::mint = mint, token::authority = authority)]
    pub token_account: Account<Token>,
    pub mint: Account<Mint>,
    pub authority: Signer,
}

impl ValidateTokenNoProgram {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
