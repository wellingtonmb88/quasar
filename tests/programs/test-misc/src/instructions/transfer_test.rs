use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferTest {
    #[account(mut)]
    pub from: Signer,
    #[account(mut)]
    pub to: UncheckedAccount,
    pub system_program: Program<System>,
}

impl TransferTest {
    #[inline(always)]
    pub fn handler(&self, amount: u64) -> Result<(), ProgramError> {
        self.system_program
            .transfer(&self.from, &self.to, amount)
            .invoke()
    }
}
