use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct InitTokenT22 {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, token::mint = mint, token::authority = payer)]
    pub token_account: Account<Token2022>,
    pub mint: Account<Mint2022>,
    pub token_program: Program<Token2022>,
    pub system_program: Program<System>,
}

impl InitTokenT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
