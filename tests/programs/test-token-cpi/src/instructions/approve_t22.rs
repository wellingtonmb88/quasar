use {
    quasar_lang::prelude::*,
    quasar_spl::{Token2022, TokenCpi},
};

#[derive(Accounts)]
pub struct ApproveT22 {
    pub authority: Signer,
    #[account(mut)]
    pub source: Account<Token2022>,
    pub delegate: UncheckedAccount,
    pub token_program: Program<Token2022>,
}

impl ApproveT22 {
    #[inline(always)]
    pub fn handler(&self, amount: u64) -> Result<(), ProgramError> {
        self.token_program
            .approve(&self.source, &self.delegate, &self.authority, amount)
            .invoke()
    }
}
