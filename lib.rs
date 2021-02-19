#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod negotiate {

    use ink_storage::{
        collections::{
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    #[derive(scale::Encode, scale::Decode, 
        SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct ConjunctionDataMessage {
        pub object1_norad_id: i32,
        pub object2_norad_id: i32,
        pub collision_probabilty: i32,
        pub time_of_closest_pass: i32
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Negotiate {
        // Who owns this contract i.e. ESA
        owner: AccountId,
        // All the messages
        cdms: StorageVec<ConjunctionDataMessage>,
        // Who can be a provider of Conjunction messages
        ca_providers: StorageVec<AccountId>
    }

    impl Negotiate {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                owner: Self::env().caller(),
                cdms: Default::default(), 
                ca_providers: Default::default(), 
            }
        }

        #[ink(message)]
        pub fn add_ca_provider(&mut self, 
            provider_account: AccountId) {

            // Only the owner of this smart contract 
            // can call this method.
            assert_eq!(self.env().caller(), self.owner);

            // Add this account to our list of providers
            self.ca_providers.push(provider_account);
        }

        #[ink(message)]
        pub fn submit_cdm(&mut self, 
            cdm: ConjunctionDataMessage) {
            
            if self.ca_providers.iter().any(|&account| 
                account == self.env().caller()) {
                self.cdms.push(cdm);
            }
        }
    }

    /// Unit tests.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope 
        /// so we can use them here.
        use super::*;

        use ink_lang as ink;

        /// The default constructor does its job.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let negotiator = Negotiate::new();

            assert_eq!(0, negotiator.cdms.len());

            assert_eq!(0, negotiator.ca_providers.len());
        }

        /// Only the contract owner can add providers
        #[ink::test]
        #[should_panic]
        fn adding_providers_with_wrong_account_panics() {
            // Constructor works.
            let mut negotiator = Negotiate::new();

            let accounts =
                ink_env::test::default_accounts::
                    <ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            // Get contract address
            let callee = ink_env::account_id::
                <ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(
            ink_env::call::Selector::new([0x00; 4])); 
            data.push_arg(&accounts.bob);
            // Push the new context to set Bob as caller
            ink_env::test::push_execution_context::
                <ink_env::DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            // This will panic
            negotiator.add_ca_provider(accounts.bob);
        }
        /// Only the contract owner can add providers
        #[ink::test]
        fn adding_providers_and_cdms_works() {
            // Constructor works.
            let mut negotiator = Negotiate::new();

            let accounts =
                ink_env::test::default_accounts::
                    <ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            // This will panic
            negotiator.add_ca_provider(accounts.frank);

            assert_eq!(1, negotiator.ca_providers.len());

            // Switch over to frank
            let callee = ink_env::account_id::
                <ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call
            let mut data =
                ink_env::test::CallData::new(
            ink_env::call::Selector::new([0x00; 4]));
            data.push_arg(&accounts.bob);
            // Push the new context to set Bob as caller
            ink_env::test::push_execution_context::
                <ink_env::DefaultEnvironment>(
                accounts.frank,
                callee,
                1000000,
                1000000,
                data,
            );

            negotiator.submit_cdm(ConjunctionDataMessage {
                object1_norad_id: 1234,
                object2_norad_id: 5678,
                collision_probabilty: 50,
                time_of_closest_pass: 123345567
            });

            assert_eq!(1, negotiator.cdms.len());
        }
    }
}
