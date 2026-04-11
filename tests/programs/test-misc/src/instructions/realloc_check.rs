use {crate::state::SimpleAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
#[instruction(new_space: u64)]
pub struct ReallocCheck {
    #[account(mut, realloc = new_space as usize)]
    pub account: Account<SimpleAccount>,
    #[account(mut)]
    pub payer: Signer,
    pub system_program: Program<System>,
}

impl ReallocCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
