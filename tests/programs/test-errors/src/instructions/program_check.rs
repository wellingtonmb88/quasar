use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct ProgramCheck {
    pub program: Program<System>,
}

impl ProgramCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
