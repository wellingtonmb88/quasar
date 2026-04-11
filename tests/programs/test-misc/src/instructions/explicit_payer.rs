use {
    crate::state::{ExplicitPayerAccount, ExplicitPayerAccountInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct ExplicitPayer {
    #[account(mut)]
    pub funder: Signer,
    #[account(mut, init, payer = funder, seeds = ExplicitPayerAccount::seeds(funder), bump)]
    pub account: Account<ExplicitPayerAccount>,
    pub system_program: Program<System>,
}

impl ExplicitPayer {
    #[inline(always)]
    pub fn handler(&mut self, value: u64, bumps: &ExplicitPayerBumps) -> Result<(), ProgramError> {
        self.account.set_inner(ExplicitPayerAccountInner {
            authority: *self.funder.address(),
            value,
            bump: bumps.account,
        });
        Ok(())
    }
}
