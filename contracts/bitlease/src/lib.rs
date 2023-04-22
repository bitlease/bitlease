#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bitlease_contract {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[derive(Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Lender {
        address: AccountId,
        lend_pools: Mapping<Currency, Balance>,
    }

    #[derive(Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Borrower {
        address: AccountId,
        loans: Mapping<Currency, Balance>,

    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std", 
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Currency {
        ASTAR,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if Currency doesn't match
        NoMatchingCurrency,
        /// Returned if not enough balance 
        InsufficientBalance,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct BitleaseContract{
        borrowers: Vec<Borrower>,
        lenders: Vec<Lender>,
        assets: Mapping<Currency, Balance>,
        interest_rate: u32,
    }

    /// Specify the result type.
    pub type Result<T> = core::result::Result<T, Error>;


    impl BitleaseContract{
        /// Constructor that initializes the contract.
        #[ink(constructor)]
        pub fn new(interest_rate: u32) -> Self {
            Self {
                borrowers: Default::default(),
                lenders: Default::default(),
                assets: Default::default(),
                interest_rate,
                }

        }
    
        
        #[ink(message, payable)]
        pub fn lend(&mut self, currency: Currency, amount: Balance) {
            // Gets the AccountId
            let caller = self.env().caller();

            // Panics if the amount is more or equal the account balance of caller
            assert!(amount >= self.env().balance(), "Insufficient Balance to lend!");
            
            // Gets only Lender in vector with that AccountId
            let mut lender = self.lenders.iter().find(|p| p.address == caller).unwrap();
            
            if let Some(b) = lender.lend_pools.get(&currency) {
                // Updates the balance 
                lender.lend_pools.insert(currency, &(b + amount));
            } else {
                // Creates entry 
                lender.lend_pools.insert(currency, &amount);
            }

            // Updates Pool 
            let pool_currency = self.assets.get(&currency);

            if let Some(b) =  pool_currency{
                // Updates the total 
                self.assets.insert(currency, &(b + amount));
            } else {
                // Creates entry 
                self.assets.insert(currency, &amount);
            }

        }
       
        #[ink(message)]
        pub fn borrow(&mut self, downpaymentCurrency: Currency, downpaymentAmount: Balance, borrowCurrency: Currency, borrowAmount: Balance) -> Result<()> {
            // Ensure the currency of the borrower and the lender are the same 
            if downpaymentCurrency != borrowCurrency{
                return Err(Error::NoMatchingCurrency);
            }
            // Check if the borrower has enough funds (for implementing collateral)
            if downpaymentAmount >= self.env().balance() {
                return Err(Error::InsufficientBalance);
            }
            // Gets the AccountId
            let caller = self.env().caller();

            // Gets only Borrower in vector with that AccountId
            let mut borrower = self.borrowers.iter().find(|p| p.address == caller).unwrap();
            
            if let Some(b) = borrower.loans.get(&borrowCurrency) {
                // Updates the balance 
                borrower.loans.insert(borrowCurrency, &(b + borrowAmount));
            } else {
                // Creates entry 
                borrower.loans.insert(borrowCurrency, &borrowAmount);
            }

            // Updates Pool 
            let pool_currency = self.assets.get(&borrowCurrency);

            if let Some(b) =  pool_currency{
                // Updates the total 
                self.assets.insert(borrowCurrency, &(b - borrowAmount));
            } else {
                // Creates entry 
                self.assets.insert(borrowCurrency, &borrowAmount);
            }
            Ok(())
        }

    }
}
