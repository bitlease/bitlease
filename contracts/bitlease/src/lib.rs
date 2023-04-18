#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bitlease_contract {
    use ink::storage::Mapping;

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Clone, Copy, PartialEq, Eq)]
    pub struct Token {
        token_address: AccountId,
        ltv: u256,
        stable_rate: u256,
        name: Vec<u8>,
    }
    
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Currency {
        BITCOIN, 
        ETHEREUM,
        TETHER,
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
        total_amount: Balance,
        /// Mapping from lender to amount lent 
        lenders: Mapping<AccountId, Balance>,
        /// Mapping from borrower to amount of collateral
        borrowers: Mapping<AccountId, Balance>,
    }

    /// Specify the result type.
    pub type Result<T> = core::result::Result<T, Error>;


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
        pub fn lend(&mut self, currency: Currency, amount: Balance) {
            // Gets the AccountId
            let lender = self.env().caller();
            // Panics if the amount is more or equal the account balance of caller
            assert!(amount >= self.env().balance(), "Insufficient Balance to lend!");
            // Add the lender and their amount to the hashmap
            self.lenders.insert(lender, &amount);

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
            // Adds the borrowed amount to the total 
            self.total_amount += borrowAmount;
            // Gets the AccountId
            let borrower = self.env().caller();
            // Add the borrower and their amount to the hashmap
            self.borrowers.insert(borrower, &downpaymentAmount);
            Ok(())
        }

    }
}
