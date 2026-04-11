use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct InitIfNeededMintT22 {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init_if_needed, mint::decimals = 6, mint::authority = mint_authority)]
    pub mint: Account<Mint2022>,
    pub mint_authority: Signer,
    pub token_program: Program<Token2022>,
    pub system_program: Program<System>,
}

impl InitIfNeededMintT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
