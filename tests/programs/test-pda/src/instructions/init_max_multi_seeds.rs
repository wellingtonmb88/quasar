use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct InitMaxMultiSeeds {
    #[account(mut)]
    pub payer: Signer,
    pub authority: Signer,
    #[account(
        seeds = [
            b"max", b"max", b"max", b"max", b"max",
            b"max", b"max", b"max", b"max", b"max",
            b"max", b"max", b"max", b"max", b"max",
        ],
        bump
    )]
    pub complex: UncheckedAccount,
    pub system_program: Program<System>,
}

impl InitMaxMultiSeeds {
    #[inline(always)]
    pub fn handler(&mut self) -> Result<(), ProgramError> {
        Ok(())
    }
}
