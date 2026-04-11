use {
    quasar_lang::prelude::*,
    quasar_spl::{Mint2022, Token2022, TokenCpi},
};

#[derive(Accounts)]
pub struct MintToT22 {
    pub authority: Signer,
    #[account(mut)]
    pub mint: Account<Mint2022>,
    #[account(mut)]
    pub to: Account<Token2022>,
    pub token_program: Program<Token2022>,
}

impl MintToT22 {
    #[inline(always)]
    pub fn handler(&self, amount: u64) -> Result<(), ProgramError> {
        self.token_program
            .mint_to(&self.mint, &self.to, &self.authority, amount)
            .invoke()
    }
}
