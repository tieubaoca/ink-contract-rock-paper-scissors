#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use scale::{Decode, Encode};
use ink_storage::traits::{SpreadAllocate, SpreadLayout, PackedLayout};
use ink_prelude::string::String;
// use std::collections::HashMap;
// #[cfg_attr(
//     feature = "std",
//     derive(
//         Debug,
//         PartialEq,
//         Eq,
//         TypeInfo,
//         ink_storage::traits::StorageLayout
//     )
// )]
// pub struct Board{
//     players: HashMap<AccountId, u8>,
//     row: u32,
//     col: u32,
//     ticked: HashMap<u32, HashMap<u32, u8>>,

// }
#[derive(Clone, Debug, Encode, Decode, SpreadLayout, PackedLayout)]
pub enum ChooseOption {
    Rock,
    Paper,
    Scissors,
}



impl ChooseOption {
    fn to_option(num: u8) -> Option<ChooseOption> {
        match num {
            0 => Some(ChooseOption::Rock),
            1 => Some(ChooseOption::Paper),
            2 => Some(ChooseOption::Scissors),
            _ => None,
        }
    }

    fn find_winner(&self, compare_option: &ChooseOption) -> String {
        match self {
            ChooseOption::Rock => match compare_option {
                ChooseOption::Rock => String::from("Draw"),
                ChooseOption::Paper => String::from("Lose"),
                ChooseOption::Scissors => String::from("Win"),
            },
            ChooseOption::Paper => match compare_option {
                ChooseOption::Rock => String::from("Win"),
                ChooseOption::Paper => String::from("Draw"),
                ChooseOption::Scissors => String::from("Lose"),
            },
            ChooseOption::Scissors => match compare_option {
                ChooseOption::Rock => String::from("Lose"),
                ChooseOption::Paper => String::from("Win"),
                ChooseOption::Scissors => String::from("Draw"),
            },
        }
    }
}

#[ink::contract]
mod my_substrate_contract {
    use super::*;

    #[ink(event)]
    pub struct Player{
        player: AccountId,
        result: String
    }
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    // #[ink(storage)]
    // #[derive(SpreadAllocate)]
    // pub struct MySubstrateContract {
    //     /// Stores a single `bool` value on the storage.
    //     cells: u64,
    //     boards: Mapping<u64, Board>,

    // }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct MySubstrateContract {}

    impl MySubstrateContract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new()
        // }

        fn random(&self) -> Option<ChooseOption> {
            let (hash, _) = self.env().random(self.env().caller().as_ref());
            let random = u128::from_be_bytes(hash.as_ref()[0..16].try_into().unwrap()) % 3;
            ChooseOption::to_option(random as u8)
        }

        #[ink(message)]
        pub fn play(&mut self, choose_option: u8) {
            let random = self.random().expect("random error");
            let choose = ChooseOption::to_option(choose_option).expect("choice error");
            let result = choose.find_winner(&random);
            self.env().emit_event(Player{
                player: self.env().caller(), 
                result: result
            })
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
    fn test1() {
        let mut my_substrate_contract = MySubstrateContract::new();
        my_substrate_contract.play(0u8);
    }
        // /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let my_substrate_contract = MySubstrateContract::default();
        //     assert_eq!(my_substrate_contract.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut my_substrate_contract = MySubstrateContract::new(false);
        //     assert_eq!(my_substrate_contract.get(), false);
        //     my_substrate_contract.flip();
        //     assert_eq!(my_substrate_contract.get(), true);
        // }
    }
}
