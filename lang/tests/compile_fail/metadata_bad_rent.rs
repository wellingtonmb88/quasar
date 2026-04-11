#![allow(unexpected_cfgs)]
use quasar_lang::prelude::*;
use quasar_spl::{metadata::MetadataProgram, Mint, Token};

solana_address::declare_id!("11111111111111111111111111111112");

#[derive(Accounts)]
pub struct BadMetadataRent {
    #[account(mut)]
    pub payer: Signer,
    pub mint_authority: Signer,
    #[account(
        mut,
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority,
        metadata::name = b"Test NFT",
        metadata::symbol = b"TNFT",
        metadata::uri = b"https://example.com/nft.json",
    )]
    pub mint: Account<Mint>,
    #[account(mut)]
    pub metadata: UncheckedAccount,
    pub metadata_program: Program<MetadataProgram>,
    pub token_program: Program<Token>,
    pub system_program: Program<System>,
    pub rent: UncheckedAccount,
}

fn main() {}
