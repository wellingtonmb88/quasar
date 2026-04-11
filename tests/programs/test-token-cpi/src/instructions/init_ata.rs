use {
    quasar_lang::prelude::*,
    quasar_spl::{AssociatedTokenProgram, Mint, Token},
};

#[derive(Accounts)]
pub struct InitAta {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, associated_token::mint = mint, associated_token::authority = wallet)]
    pub ata: Account<Token>,
    pub wallet: Signer,
    pub mint: Account<Mint>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
    pub ata_program: Program<AssociatedTokenProgram>,
}

impl InitAta {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
