use {
    crate::state::{RentCalcSnapshot, RentCalcSnapshotInner},
    quasar_lang::{
        prelude::*,
        sysvars::{rent::Rent, Sysvar as _},
    },
};

#[derive(Accounts)]
pub struct ReadRentCalc {
    #[account(mut)]
    pub payer: Signer,
    #[account(mut, init, payer = payer, seeds = RentCalcSnapshot::seeds(), bump)]
    pub snapshot: Account<RentCalcSnapshot>,
    pub system_program: Program<System>,
}

impl ReadRentCalc {
    #[inline(always)]
    pub fn handler(&mut self, data_len: u64) -> Result<(), ProgramError> {
        let rent = Rent::get()?;
        let min_balance = rent.minimum_balance_unchecked(data_len as usize);
        self.snapshot
            .set_inner(RentCalcSnapshotInner { min_balance });
        Ok(())
    }
}
