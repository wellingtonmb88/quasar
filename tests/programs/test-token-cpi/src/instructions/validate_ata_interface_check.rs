use {
    quasar_lang::prelude::*,
    quasar_spl::{InterfaceAccount, Mint, Token, TokenInterface},
};

#[derive(Accounts)]
pub struct ValidateAtaInterfaceCheck {
    #[account(associated_token::mint = mint, associated_token::authority = wallet)]
    pub ata: InterfaceAccount<Token>,
    pub mint: InterfaceAccount<Mint>,
    pub wallet: Signer,
    pub token_program: Interface<TokenInterface>,
}

impl ValidateAtaInterfaceCheck {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        Ok(())
    }
}
