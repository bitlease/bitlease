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
    pub struct Pools { 
        pool: Mapping<Currency, Balance>,
    }

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
        assets: Vec<Pools>
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
            let mut lender = self.lenders.iter().find(|p| p.address == caller);
            
            if let Some(b) = lender.lend_pools.get(&currency) {
                // Updates the balance 
                lender.lend_pools.insert(currency, &(b + amount));
            } else {
                // Creates entry 
                lender.lend_pools.insert(currency, amount);
            }

            // Updates Pool 
            let pool_currency = self.assets.pool.get(&currency)

            if let Some(b) =  pool_currency{
                // Updates the total 
                pool_currency.pool.insert(currency, &(b + amount));
            } else {
                // Creates entry 
                pool_currency.pool.insert(currency, amount);
            }

        }

        #[ink(message)]
        pub fn borrow(&mut self, downpaymentCurrency: Currency, downpaymentAmount: Balance, borrowCurrency: Currency, borrowAmount: Balance) -> Result<()> {
            // Ensure the currency of the borrower and the lender are the same 
            if downpaymentCurrency != borrowCurrency{
                return Err(Error::NoMatchingCurrency);
            }
            // Check if the borrower has enough funds 
            if downpaymentAmount >= self.env().balance() {
                return Err(Error::InsufficientBalance);
            }
            // Gets the AccountId
            let caller = self.env().caller();
            // Instantiate a Borrower
            let borrower = Borrower {
                collateral_currency: downpaymentCurrency,
                collateral_amount: downpaymentAmount,
                borrowed_amount: borrowAmount,
            };
            // Add the AccountId and borrower to the Mapping
            self.borrowers.insert(caller, &borrower);
            Ok(())
        }

    }
}
