use {crate::state::DynamicAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct DynamicMutate<'account> {
    #[account(mut)]
    pub account: Account<DynamicAccount<'account>>,
    #[account(mut)]
    pub payer: Signer,
    pub system_program: Program<System>,
}

impl DynamicMutate<'_> {
    #[inline(always)]
    pub fn handler(&mut self, new_name: &str) -> Result<(), ProgramError> {
        self.account.set_name(&self.payer, new_name)?;
        Ok(())
    }
}
