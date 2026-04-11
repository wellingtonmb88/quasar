use {crate::events::HeapTestEvent, quasar_lang::prelude::*};

#[derive(Accounts)]
pub struct EmitEventOk {
    pub signer: Signer,
}

impl EmitEventOk {
    #[inline(always)]
    pub fn handler(&self) -> Result<(), ProgramError> {
        emit!(HeapTestEvent { value: 42 });
        Ok(())
    }
}
