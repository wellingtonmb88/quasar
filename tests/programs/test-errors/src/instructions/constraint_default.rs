use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct ConstraintDefault {
    #[account(constraint = false)]
    pub target: SystemAccount,
}

impl ConstraintDefault {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
