use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct InitIfNeededMintWithFreezeT22 {
    #[account(mut)]
    pub payer: Signer,
    #[account(
        mut,
        init_if_needed,
        mint::decimals = 6,
        mint::authority = mint_authority,
        mint::freeze_authority = freeze_authority
    )]
    pub mint: Account<Mint2022>,
    pub mint_authority: Signer,
    pub freeze_authority: UncheckedAccount,
    pub token_program: Program<Token2022>,
    pub system_program: Program<System>,
}

impl InitIfNeededMintWithFreezeT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
