use {crate::state::ErrorTestAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct AccountCheckIx {
    pub account: Account<ErrorTestAccount>,
}

impl AccountCheckIx {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
