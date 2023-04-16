#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bitlease_contract {
    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Currency {
        BITCOIN, 
        ETHEREUM,
        TETHER,
    }

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
            /*let mut borrowers = Mapping::new();
            let caller = Self::env().caller();
            borrowers.insert(caller, &collateral);
            */
            Self {
                total_amount: 0,
                lenders: Default::default(),
                borrowers: Default::default(),
                }

        }
    
        
        #[ink(message)]
        pub fn lend(&mut self, lender: AccountId, currency: Currency, amount: Balance) {
            //let caller = self.env().caller();
            /// Panics if the amount is more or equal the account balance of caller
            assert!(amount >= self.env().balance(), "Insufficient Balance to lend!");
            // Add the lender and their amount to the hashmap
            self.lenders.insert(lender, &amount);

        }

    }
}


