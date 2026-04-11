use {
    quasar_lang::prelude::*,
    quasar_spl::{AssociatedTokenProgram, Mint, Token},
};

#[derive(Accounts)]
pub struct InitIfNeededAta {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init_if_needed, associated_token::mint = mint, associated_token::authority = wallet)]
    pub ata: Account<Token>,
    pub wallet: Signer,
    pub mint: Account<Mint>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
    pub ata_program: Program<AssociatedTokenProgram>,
}

impl InitIfNeededAta {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
