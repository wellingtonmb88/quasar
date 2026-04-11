use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint, Token},
};

/// Tests closing a token account via the `close =` attribute.
/// The macro's epilogue calls `Account::close(dest)` which zeros the account,
/// transfers lamports, and reassigns to the system program.
#[derive(Accounts)]
pub struct CloseToken {
    pub authority: Signer,
    #[account(mut, close = destination, token::mint = mint, token::authority = authority)]
    pub token_account: Account<Token>,
    pub mint: Account<Mint>,
    /// CHECK: destination may alias authority (close sends lamports to it).
    #[account(mut, dup)]
    pub destination: UncheckedAccount,
    pub token_program: Program<Token>,
}

impl CloseToken {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
