#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bitlease_contract {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct BitleaseContract{
        total_amount: Balance,
        /// Mapping from lender to amount lent 
        lenders: Mapping<AccountId, Balance>,
        /// Mapping from borrower to amount of collateral
        borrowers: Mapping<AccountId, Balance>,
    }

    impl BitleaseContract{
        /// Constructor that initializes the contract.
        #[ink(constructor)]
        pub fn new(collateral: u128) -> Self {
            let mut borrowers = Mapping::default();
            let caller = Self::env().caller();
            borrowers.insert(caller, &collateral);

            Self {
                total_amount: collateral,
                lenders: Default::default(),
                borrowers,
                }

        }
    
        

    }
}


