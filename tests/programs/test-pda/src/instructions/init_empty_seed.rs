use {
    crate::state::{EmptySeedAccount, EmptySeedAccountInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitEmptySeed {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, payer = payer, seeds = EmptySeedAccount::seeds(), bump)]
    pub empty: Account<EmptySeedAccount>,
    pub system_program: Program<System>,
}

impl InitEmptySeed {
    #[inline(always)]
    pub fn handler(&mut self, bumps: &InitEmptySeedBumps) -> Result<(), ProgramError> {
        self.empty
            .set_inner(EmptySeedAccountInner { bump: bumps.empty });
        Ok(())
    }
}
