#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bitlease_contract {
    use ink::storage::Mapping;
    use ink::env::caller;

    #[ink(storage)]
    pub struct BitleaseContract<const INTEREST: u32>{
        /// Mapping to store the lender and their respective amount
        lender: Mapping<AccountId, (Currency, Balance)>,
        /// Mapping to store the borrower and their respective amount
        borrower: Mapping<AccountId, (Currency, Balance)>,
        /// The currency the borrower wants to invest in 
        currency_to_invest: Currency,
    }

    
}