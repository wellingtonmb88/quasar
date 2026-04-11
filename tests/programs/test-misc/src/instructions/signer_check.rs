use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct SignerCheck {
    pub signer: Signer,
}

impl SignerCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
