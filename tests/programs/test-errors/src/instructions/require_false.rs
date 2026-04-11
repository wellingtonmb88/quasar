use {crate::errors::TestError, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct RequireFalse {
    pub signer: Signer,
}

impl RequireFalse {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        require!(false, TestError::RequireFailed);
        Ok(())
    }
}
