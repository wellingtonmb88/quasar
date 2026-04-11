use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022},
};

#[derive(Accounts)]
pub struct ValidateMintWithFreeze2022Check {
    #[account(mint::authority = mint_authority, mint::decimals = 6, mint::freeze_authority = freeze_authority)]
    pub mint: Account<Mint2022>,
    pub mint_authority: Signer,
    pub freeze_authority: UncheckedAccount,
    pub token_program: Program<Token2022>,
}

impl ValidateMintWithFreeze2022Check {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
