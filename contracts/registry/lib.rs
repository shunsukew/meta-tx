#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[ink::contract]
mod registry {
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use openbrush::{
        traits::{
            Storage,
        },
    };
    use meta_tx_context::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Registry {
        #[storage_field]
        meta_tx_context: meta_tx_context::Data,

        names: Mapping<AccountId, String>,
        owners: Mapping<String, AccountId>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        NameTaken,
        AlreadyRegistered,
        NameNotRegistered,
    }

    impl meta_tx_context::MetaTxContext for Registry {}
    impl meta_tx_context::Internal for Registry {}

    impl Registry {
        #[ink(constructor)]
        pub fn new(trusted_forwarder: AccountId) -> Self {
            Self {
                meta_tx_context: meta_tx_context::Data {
                    trusted_forwarder: Some(trusted_forwarder),
                },
                names: Mapping::new(),
                owners: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn register(&mut self, name: String, data: Vec<u8>) -> Result<(), Error> {
            if self.owners.contains(name.clone()) {
                return Err(Error::NameTaken);
            };
            let caller = self._caller(data);
            if self.names.contains(caller) {
                return Err(Error::AlreadyRegistered);
            }

            self.names.insert(caller, &name);
            self.owners.insert(name, &caller);

            Ok(())
        }

        #[ink(message)]
        pub fn unregister(&mut self, data: Vec<u8>) -> Result<(), Error> {
            let caller = self._caller(data);
            let name = self.names.get(caller).ok_or(Error::NameNotRegistered)?;

            self.names.remove(&caller);
            self.owners.remove(name);

            Ok(())
        }

        #[ink(message)]
        pub fn get_name(&self, account_id: AccountId) -> Option<String> {
            self.names.get(account_id)
        }

        #[ink(message)]
        pub fn get_owner(&self, name: String) ->Option<AccountId> {
            self.owners.get(name)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        #[ink::test]
        fn register_works() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new([0x0; 32].into());
            assert_eq!(registry.register(String::from("alice"), vec![]), Ok(()));
            assert_eq!(registry.get_owner(String::from("alice")), Some(accounts.alice));
            assert_eq!(registry.get_name(accounts.alice), Some(String::from("alice")));
        }

        #[ink::test]
        fn register_twice_fail() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new([0x0; 32].into());
            assert_eq!(registry.register(String::from("alice"), vec![]), Ok(()));
            assert_eq!(registry.register(String::from("alice_2"), vec![]), Err(Error::AlreadyRegistered));
        }

        #[ink::test]
        fn register_taken_name_fail() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new([0x0; 32].into());
            assert_eq!(registry.register(String::from("test"), vec![]), Ok(()));

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(registry.register(String::from("test"), vec![]), Err(Error::NameTaken));
        }

        #[ink::test]
        fn unregister_works() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new([0x0; 32].into());
            assert_eq!(registry.register(String::from("alice"), vec![]), Ok(()));
            assert_eq!(registry.unregister(vec![]), Ok(()));
            assert_eq!(registry.get_name(accounts.alice), None);
        }

        #[ink::test]
        fn unregister_no_record_fail() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new([0x0; 32].into());
            assert_eq!(registry.unregister(vec![]), Err(Error::NameNotRegistered));
        }
    }
}
