#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod registry {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Registry {
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

    impl Registry {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                names: Mapping::new(),
                owners: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn register(&mut self, name: String) -> Result<(), Error> {
            if self.owners.contains(name.clone()) {
                return Err(Error::NameTaken);
            };
            let caller = self.env().caller();
            if self.names.contains(caller) {
                return Err(Error::AlreadyRegistered);
            }

            self.names.insert(caller, &name);
            self.owners.insert(name, &caller);

            Ok(())
        }

        #[ink(message)]
        pub fn unregister(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
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

            let mut registry = Registry::new();
            assert_eq!(registry.register(String::from("alice")), Ok(()));
            assert_eq!(registry.get_owner(String::from("alice")), Some(accounts.alice));
            assert_eq!(registry.get_name(accounts.alice), Some(String::from("alice")));
        }

        #[ink::test]
        fn register_twice_fail() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new();
            assert_eq!(registry.register(String::from("alice")), Ok(()));
            assert_eq!(registry.register(String::from("alice_2")), Err(Error::AlreadyRegistered));
        }

        #[ink::test]
        fn register_taken_name_fail() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new();
            assert_eq!(registry.register(String::from("test")), Ok(()));

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(registry.register(String::from("test")), Err(Error::NameTaken));
        }

        #[ink::test]
        fn unregister_works() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new();
            assert_eq!(registry.register(String::from("alice")), Ok(()));
            assert_eq!(registry.unregister(), Ok(()));
            assert_eq!(registry.get_name(accounts.alice), None);
        }

        #[ink::test]
        fn unregister_no_record_fail() {
            let accounts = default_accounts();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut registry = Registry::new();
            assert_eq!(registry.unregister(), Err(Error::NameNotRegistered));
        }
    }
}
