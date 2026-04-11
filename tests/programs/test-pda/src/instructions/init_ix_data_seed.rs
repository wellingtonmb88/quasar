use {
    crate::state::{IndexedAccount, IndexedAccountInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(index: u64)]
pub struct InitIxDataSeed {
    #[account(mut)]
    pub payer: Signer,
    pub authority: Signer,
    #[account(mut, init, payer = payer, seeds = IndexedAccount::seeds(authority, index), bump)]
    pub item: Account<IndexedAccount>,
    pub system_program: Program<System>,
}

impl InitIxDataSeed {
    #[inline(always)]
    pub fn handler(&mut self, index: u64, bumps: &InitIxDataSeedBumps) -> Result<(), ProgramError> {
        self.item.set_inner(IndexedAccountInner {
            authority: *self.authority.address(),
            index,
            bump: bumps.item,
        });
        Ok(())
    }
}
