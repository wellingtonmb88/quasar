use {crate::state::SimpleAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct UpdateHasOne {
    pub authority: Signer,
    #[account(has_one = authority, seeds = SimpleAccount::seeds(authority), bump = account.bump)]
    pub account: Account<SimpleAccount>,
}

impl UpdateHasOne {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
