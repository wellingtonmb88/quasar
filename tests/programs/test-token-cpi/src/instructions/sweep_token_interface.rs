use {
    quasar_lang::prelude::*,
    quasar_spl::{InterfaceAccount, Mint, Token, TokenInterface},
};

#[derive(Accounts)]
pub struct SweepTokenInterface {
    pub authority: Signer,
    #[account(mut, sweep = receiver, token::mint = mint, token::authority = authority)]
    pub source: InterfaceAccount<Token>,
    #[account(mut)]
    pub receiver: InterfaceAccount<Token>,
    pub mint: InterfaceAccount<Mint>,
    pub token_program: Interface<TokenInterface>,
}

impl SweepTokenInterface {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
