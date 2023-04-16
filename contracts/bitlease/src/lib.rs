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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, serde::Serialize, serde::Deserialize))]
    pub enum Currency {
        DEFAULT,
        BITCOIN, 
        ETHEREUM,
        TETHER,
    }

    impl<const INTEREST: u32>BitleaseContract<INTEREST>{
        /// Constructor that initializes the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                let mut lender: Mapping::default();
                let mut borrower: Mapping::default();
                let caller = Self::env().caller();
                borrower.insert(&caller, &0);

                Self {
                    lender,
                    borrower,
                    currency_to_invest: Currency::DEFAULT,
                }

            }
        }
    }

    #[ink(message)]
    pub fn get_lender_amount(&self, lender: AccountId) -> (Currency, Balance) {
        *self.lenders.get(&lender).unwrap_or(&(Currency, 0))
    }

    #[ink(message)]
    pub fn get_borrower_amount(&self, borrower: AccountId) -> (Currency, Balance) {
        *self.borrowers.get(&borrower).unwrap_or(&(Currency, 0))
    }


}


    
/*
For eg. When a lender has a deposit to make, it should go to a pool
function lend(lenderid, currency, amount)
function borrow(borrowerid, downpaymentCurrency, downpaymentAmount,  borrowCurrency, borrowAmount, interest_rate) 
We need a few hashmaps that store the things as key value

calculateInterest function

lender's amount goes to the total pool.. borrower borrors form the total pool
that pool is a hashmap or multiple hashmaps that tracks the amounts for ecah borrowers and lenders

*/