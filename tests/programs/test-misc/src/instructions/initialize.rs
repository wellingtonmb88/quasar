use {
    crate::state::{SimpleAccount, SimpleAccountInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitializeSimple {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, payer = payer, seeds = SimpleAccount::seeds(payer), bump)]
    pub account: Account<SimpleAccount>,
    pub system_program: Program<System>,
}

impl InitializeSimple {
    #[inline(always)]
    pub fn handler(
        &mut self,
        value: u64,
        bumps: &InitializeSimpleBumps,
    ) -> Result<(), ProgramError> {
        self.account.set_inner(SimpleAccountInner {
            authority: *self.payer.address(),
            value,
            bump: bumps.account,
        });
        Ok(())
    }
}
