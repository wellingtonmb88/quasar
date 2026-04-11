use {
    crate::{events::SimpleEvent, EventAuthority, QuasarTestEvents},
    quasar_lang::prelude::*,
};

#[derive(Accounts)]
pub struct EmitViaCpi {
    pub signer: Signer,
    pub event_authority: EventAuthority,
    pub program: Program<QuasarTestEvents>,
}

impl EmitViaCpi {
    #[inline(always)]
    pub fn handler(&self, value: u64) -> Result<(), ProgramError> {
        emit_cpi!(SimpleEvent { value })?;
        Ok(())
    }
}
