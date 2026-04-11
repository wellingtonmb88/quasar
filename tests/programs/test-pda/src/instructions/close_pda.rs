use {crate::state::UserAccount, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct ClosePda {
    #[account(mut)]
    pub authority: Signer,
    #[account(
        mut,
        has_one = authority,
        close = authority,
        seeds = UserAccount::seeds(authority),
        bump = user.bump
    )]
    pub user: Account<UserAccount>,
}

impl ClosePda {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
