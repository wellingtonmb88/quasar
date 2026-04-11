use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

/// Tests sweep without close — transfers all remaining tokens at end of
/// instruction.
#[derive(Accounts)]
pub struct SweepToken {
    pub authority: Signer,
    #[account(mut, sweep = receiver, token::mint = mint, token::authority = authority)]
    pub source: Account<Token>,
    #[account(mut)]
    pub receiver: Account<Token>,
    pub mint: Account<Mint>,
    pub token_program: Program<Token>,
}

impl SweepToken {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
