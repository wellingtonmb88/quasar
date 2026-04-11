use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct SignerNeeded {
    pub signer: Signer,
}

impl SignerNeeded {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
