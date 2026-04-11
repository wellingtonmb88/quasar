use {
    quasar_lang::prelude::*,
    quasar_spl::{metadata::MetadataProgram, Mint, Token},
};

#[derive(Accounts)]
pub struct InitMintWithMetadata {
    #[account(mut)]
    pub payer: Signer,
    pub mint_authority: Signer,
    #[account(
        mut,
        init,
        mint::decimals = 0,
        mint::authority = mint_authority,
        metadata::name = b"Test NFT",
        metadata::symbol = b"TNFT",
        metadata::uri = b"https://example.com/nft.json",
        metadata::seller_fee_basis_points = 500,
        metadata::is_mutable = true,
    )]
    pub mint: Account<Mint>,
    #[account(mut)]
    pub metadata: UncheckedAccount,
    pub metadata_program: Program<MetadataProgram>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
    pub rent: Sysvar<Rent>,
}

impl InitMintWithMetadata {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
