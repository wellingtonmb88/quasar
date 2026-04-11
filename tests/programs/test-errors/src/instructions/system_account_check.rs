use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct SystemAccountCheck {
    pub account: SystemAccount,
}

impl SystemAccountCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
