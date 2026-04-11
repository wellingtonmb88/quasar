use {crate::state::SimpleAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct CloseAccount {
    #[account(mut)]
    pub authority: Signer,
    #[account(
        has_one = authority,
        close = authority,
        seeds = SimpleAccount::seeds(authority),
        bump = account.bump
    )]
    #[account(mut)]
    pub account: Account<SimpleAccount>,
}

impl CloseAccount {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
