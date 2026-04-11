use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct InitIfNeededToken {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init_if_needed, token::mint = mint, token::authority = payer)]
    pub token_account: Account<Token>,
    pub mint: Account<Mint>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
}

impl InitIfNeededToken {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
