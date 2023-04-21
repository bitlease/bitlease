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
        amount_lent: Balance,
    }

    #[derive(Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Borrower {
        collateral_currency: Currency,
        collateral_amount: Balance,
        //borrowed_currency: Currency,
        borrowed_amount: Balance,
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
        interest_rate: u32,
        /// Mapping from lender to amount lent 
        lenders: Mapping<Currency, Vec<Lender>>,
        /// Mapping from borrower to amount of collateral
        borrowers: Mapping<AccountId, Borrower>,
    }

    /// Specify the result type.
    pub type Result<T> = core::result::Result<T, Error>;


    impl BitleaseContract{
        /// Constructor that initializes the contract.
        #[ink(constructor)]
        pub fn new(interest_rate: u32) -> Self {
            /*let mut borrowers = Mapping::new();
            let caller = Self::env().caller();
            borrowers.insert(caller, &collateral);
            */
            Self {
                interest_rate,
                lenders: Default::default(),
                borrowers: Default::default(),
                }

        }
    
        
        #[ink(message)]
        pub fn lend(&mut self, currency: Currency, amount: Balance) {
            // Gets the AccountId
            let caller = self.env().caller();
            // Panics if the amount is more or equal the account balance of caller
            assert!(amount >= self.env().balance(), "Insufficient Balance to lend!");
            // Instantiate a Lender
            let lender = Lender {
                address: caller,
                amount_lent: amount,
            };
            // Creates the vector of Lenders 
            let mut lenders_currency = self.lenders.get(&currency).unwrap_or_default();
            lenders_currency.push(lender);
            // Add the lender and their currency to the Mapping
            self.lenders.insert(currency, &lenders_currency);

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
