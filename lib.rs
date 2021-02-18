#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod negotiate {

    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        Lazy,
    };

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
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
        cdms: StorageVec<ConjunctionDataMessage>,
        ca_providers: StorageVec<AccountId>
    }

    impl Negotiate {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                cdms: Default::default(), 
                ca_providers: Default::default(), 
            }
        }

        #[ink(message)]
        pub fn add_ca_provider(&mut self, provider_account: AccountId) {
            // Only the owner of this smart contract can call this method.
            assert_eq!(self.env().caller(), self.env().account_id());

            // Add this account to our list of roviders
            self.ca_providers.push(provider_account);
        }

        #[ink(message)]
        pub fn submit_cdm(&mut self, cdm: ConjunctionDataMessage) {
        }
    }
}
