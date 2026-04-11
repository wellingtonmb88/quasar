use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct InitToken {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, token::mint = mint, token::authority = payer)]
    pub token_account: Account<Token>,
    pub mint: Account<Mint>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
}

impl InitToken {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
