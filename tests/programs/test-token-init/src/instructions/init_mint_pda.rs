use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct InitMintPda {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, seeds = [b"mint", payer], bump, mint::decimals = 6, mint::authority = payer)]
    pub mint: Account<Mint>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
}

impl InitMintPda {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
