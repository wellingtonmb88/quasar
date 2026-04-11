use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct InitMintPdaT22 {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, seeds = [b"mint", payer], bump, mint::decimals = 6, mint::authority = payer)]
    pub mint: Account<Mint2022>,
    pub token_program: Program<Token2022>,
    pub system_program: Program<System>,
}

impl InitMintPdaT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
