use {crate::errors::TestError, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct ConstraintFail {
    #[account(constraint = false @ TestError::ConstraintCustom)]
    pub target: SystemAccount,
}

impl ConstraintFail {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
