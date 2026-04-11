use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

#[derive(Accounts)]
pub struct InitIfNeededMintWithFreeze {
    #[account(mut)]
    pub payer: Signer,
    #[account(
        mut,
        init_if_needed,
        mint::decimals = 6,
        mint::authority = mint_authority,
        mint::freeze_authority = freeze_authority
    )]
    pub mint: Account<Mint>,
    pub mint_authority: Signer,
    pub freeze_authority: UncheckedAccount,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
}

impl InitIfNeededMintWithFreeze {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
