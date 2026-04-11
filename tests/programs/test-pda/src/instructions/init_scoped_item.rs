use {
    crate::state::{ScopedItem, ScopedItemInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(namespace: u32)]
pub struct InitScopedItem {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, payer = payer, seeds = ScopedItem::seeds(namespace), bump)]
    pub item: Account<ScopedItem>,
    pub system_program: Program<System>,
}

impl InitScopedItem {
    pub fn handler(
        &mut self,
        namespace: u32,
        bumps: &InitScopedItemBumps,
    ) -> Result<(), ProgramError> {
        self.item.set_inner(ScopedItemInner {
            namespace,
            data: 0,
            bump: bumps.item,
        });
        Ok(())
    }
}
