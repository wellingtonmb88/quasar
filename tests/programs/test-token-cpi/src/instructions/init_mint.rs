use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct InitMintAccount {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, mint::decimals = 6, mint::authority = mint_authority)]
    pub mint: Account<Mint>,
    pub mint_authority: Signer,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
}

impl InitMintAccount {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
