#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod traits;

pub use traits::*;

use openbrush::{
    traits::{
        AccountId,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(MetaTxContextData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub trusted_forwarder: Option<AccountId>,
}

impl<T> MetaTxContext for T
where
    T: Storage<Data>,
{
    default fn get_trusted_forwarder(&self) -> Option<AccountId> {
        self.data::<Data>().trusted_forwarder
    }
}

impl<T> Internal for T
where
    T: Storage<Data>
{
    default fn _set_trusted_forwarder(&mut self, forwarder: AccountId) {
        self.data::<Data>().trusted_forwarder = Some(forwarder);
    }

    default fn _caller(&self, data: Vec<u8>) -> AccountId {
        let caller = Self::env().caller();
        if let Some(trusted_forwarder) = self.data::<Data>().trusted_forwarder {
            if caller == trusted_forwarder {
                return AccountId::try_from(data.as_slice()).unwrap();
            }
        }
        caller
    }
}
