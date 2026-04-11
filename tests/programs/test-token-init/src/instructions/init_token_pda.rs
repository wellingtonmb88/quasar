use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct InitTokenPda {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, seeds = [b"token", payer], bump, token::mint = mint, token::authority = payer)]
    pub token_account: Account<Token>,
    pub mint: Account<Mint>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
}

impl InitTokenPda {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
