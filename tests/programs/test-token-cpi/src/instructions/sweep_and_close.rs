use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

/// Tests sweep + close — transfers all tokens, then closes the account.
#[derive(Accounts)]
pub struct SweepAndClose {
    pub authority: Signer,
    #[account(mut, sweep = receiver, close = destination, token::mint = mint, token::authority = authority)]
    pub source: Account<Token>,
    #[account(mut)]
    pub receiver: Account<Token>,
    pub mint: Account<Mint>,
    #[account(mut)]
    pub destination: UncheckedAccount,
    pub token_program: Program<Token>,
}

impl SweepAndClose {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
