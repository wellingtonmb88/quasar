use {
    crate::state::{ThreeSeedAccount, ThreeSeedAccountInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitThreeSeeds {
    #[account(mut)]
    pub payer: Signer,
    pub first: Signer,
    pub second: Signer,
    #[account(mut, init, payer = payer, seeds = ThreeSeedAccount::seeds(first, second), bump)]
    pub triple: Account<ThreeSeedAccount>,
    pub system_program: Program<System>,
}

impl InitThreeSeeds {
    #[inline(always)]
    pub fn handler(&mut self, bumps: &InitThreeSeedsBumps) -> Result<(), ProgramError> {
        self.triple.set_inner(ThreeSeedAccountInner {
            first: *self.first.address(),
            second: *self.second.address(),
            bump: bumps.triple,
        });
        Ok(())
    }
}
