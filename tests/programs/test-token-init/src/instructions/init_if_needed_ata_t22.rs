use {
    quasar_lang::prelude::*,
    quasar_spl::{AssociatedTokenProgram, Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct InitIfNeededAtaT22 {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init_if_needed, associated_token::mint = mint, associated_token::authority = wallet)]
    pub ata: Account<Token2022>,
    pub wallet: Signer,
    pub mint: Account<Mint2022>,
    pub token_program: Program<Token2022>,
    pub system_program: Program<System>,
    pub ata_program: Program<AssociatedTokenProgram>,
}

impl InitIfNeededAtaT22 {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
