use {
    crate::state::{NamespaceConfig, NamespaceConfigInner},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitNsConfig {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, payer = payer, seeds = NamespaceConfig::seeds(), bump)]
    pub config: Account<NamespaceConfig>,
    pub system_program: Program<System>,
}

impl InitNsConfig {
    pub fn handler(
        &mut self,
        namespace: u32,
        bumps: &InitNsConfigBumps,
    ) -> Result<(), ProgramError> {
        self.config.set_inner(NamespaceConfigInner {
            namespace,
            bump: bumps.config,
        });
        Ok(())
    }
}
