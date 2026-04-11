use {crate::state::SimpleAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct ConstraintCheck {
    #[account(constraint = account.value > 0)]
    pub account: Account<SimpleAccount>,
}

impl ConstraintCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
