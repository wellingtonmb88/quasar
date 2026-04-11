use {crate::state::DynamicAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct DynamicAccountCheck<'account> {
    pub account: Account<DynamicAccount<'account>>,
}

impl DynamicAccountCheck<'_> {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
