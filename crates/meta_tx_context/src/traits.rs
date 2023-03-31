use openbrush::{
    traits::AccountId,
};
use ink::prelude::vec::Vec;
use openbrush::contracts::access_control::*;

#[openbrush::wrapper]
pub type MetaTxContextRef = dyn MetaTxContext;

#[openbrush::trait_definition]
pub trait MetaTxContext {
    #[ink(message)]
    fn get_trusted_forwarder(&self) -> Option<AccountId>;

    #[ink(message)]
    fn set_trusted_forwarder(&mut self, forwarder: AccountId) -> Result<(), AccessControlError>;

    fn _caller(&self, data: Vec<u8>) -> AccountId;
}
