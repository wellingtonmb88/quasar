use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct InitMintT22 {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, mint::decimals = 6, mint::authority = mint_authority)]
    pub mint: Account<Mint2022>,
    pub mint_authority: Signer,
    pub token_program: Program<Token2022>,
    pub system_program: Program<System>,
}

impl InitMintT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
