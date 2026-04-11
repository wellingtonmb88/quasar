use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct ValidateToken2022Check {
    #[account(token::mint = mint, token::authority = authority)]
    pub token_account: Account<Token2022>,
    pub mint: Account<Mint2022>,
    pub authority: Signer,
    pub token_program: Program<Token2022>,
}

impl ValidateToken2022Check {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
