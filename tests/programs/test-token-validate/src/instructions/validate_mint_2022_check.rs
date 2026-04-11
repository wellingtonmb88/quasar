use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct ValidateMint2022Check {
    #[account(mint::authority = mint_authority, mint::decimals = 6)]
    pub mint: Account<Mint2022>,
    pub mint_authority: Signer,
    pub token_program: Program<Token2022>,
}

impl ValidateMint2022Check {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
