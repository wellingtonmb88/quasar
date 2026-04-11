use {
    crate::state::{NamespaceConfig, ScopedItem},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct VerifyScopedItem {
    pub config: Account<NamespaceConfig>,
    #[account(seeds = ScopedItem::seeds(config.namespace), bump = item.bump)]
    pub item: Account<ScopedItem>,
}

impl VerifyScopedItem {
    pub fn handler(&mut self) -> Result<(), ProgramError> {
        Ok(())
    }
}
