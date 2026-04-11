use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct SweepAndCloseT22 {
    pub authority: Signer,
    #[account(mut, sweep = receiver, close = destination, token::mint = mint, token::authority = authority)]
    pub source: Account<Token2022>,
    #[account(mut)]
    pub receiver: Account<Token2022>,
    pub mint: Account<Mint2022>,
    #[account(mut)]
    pub destination: UncheckedAccount,
    pub token_program: Program<Token2022>,
}

impl SweepAndCloseT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
