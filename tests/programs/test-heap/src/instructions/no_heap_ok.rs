use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct NoHeapOk {
    pub signer: Signer,
}

impl NoHeapOk {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
