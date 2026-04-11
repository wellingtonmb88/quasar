use {
    crate::state::{SimpleAccount, SimpleAccountInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitIfNeeded {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init_if_needed, payer = payer, seeds = SimpleAccount::seeds(payer), bump)]
    pub account: Account<SimpleAccount>,
    pub system_program: Program<System>,
}

impl InitIfNeeded {
    #[inline(always)]
    pub fn handler(&mut self, value: u64, bumps: &InitIfNeededBumps) -> Result<(), ProgramError> {
        self.account.set_inner(SimpleAccountInner {
            authority: *self.payer.address(),
            value,
            bump: bumps.account,
        });
        Ok(())
    }
}
