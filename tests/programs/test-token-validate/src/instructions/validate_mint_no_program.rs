use {quasar_lang::prelude::*, quasar_spl::Mint};

/// No `token_program` field — program is known at compile time from
/// Account<Mint>.
#[derive(Accounts)]
pub struct ValidateMintNoProgram {
    #[account(mint::authority = mint_authority, mint::decimals = 6)]
    pub mint: Account<Mint>,
    pub mint_authority: Signer,
}

impl ValidateMintNoProgram {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
