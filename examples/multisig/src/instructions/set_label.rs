use {crate::state::MultisigConfig, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct SetLabel<'config> {
    #[account(mut)]
    pub creator: Signer,
    #[account(
        mut,
        has_one = creator,
        seeds = MultisigConfig::seeds(creator),
        bump = config.bump
    )]
    pub config: Account<MultisigConfig<'config>>,
    pub system_program: Program<System>,
}

impl SetLabel<'_> {
    #[inline(always)]
    pub fn update_label(&mut self, label: &str) -> Result<(), ProgramError> {
        self.config.set_label(&self.creator, label)
    }
}
