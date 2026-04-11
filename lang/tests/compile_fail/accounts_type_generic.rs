#![allow(unexpected_cfgs)]
use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct BadGenericAccount<T> {
    pub signer: Signer,
    pub _marker: core::marker::PhantomData<T>,
}

fn main() {}
