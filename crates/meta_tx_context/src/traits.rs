#![cfg_attr(not(feature = "std"), no_std)]

use openbrush::{
    traits::AccountId,
};

#[openbrush::wrapper]
pub type MetaTxContextRef = dyn MetaTxContext;

#[openbrush::trait_definition]
pub trait MetaTxContext {
    #[ink(message)]
    fn get_trusted_forwarder(&self) -> Option<AccountId>;
}

pub trait Internal {
    fn _set_trusted_forwarder(&mut self, forwarder: AccountId);

    fn _caller(&self, data: Vec<u8>) -> AccountId;
}
