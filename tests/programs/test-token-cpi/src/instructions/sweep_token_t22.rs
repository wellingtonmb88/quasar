use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct SweepTokenT22 {
    pub authority: Signer,
    #[account(mut, sweep = receiver, token::mint = mint, token::authority = authority)]
    pub source: Account<Token2022>,
    #[account(mut)]
    pub receiver: Account<Token2022>,
    pub mint: Account<Mint2022>,
    pub token_program: Program<Token2022>,
}

impl SweepTokenT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
