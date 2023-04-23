#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bitlease_contract {
    use ink::storage::Mapping;

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

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Lend {
        amount: Balance,
        currency: Currency,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Borrow {
        amount: Balance,
        currency: Currency, 
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct BitleaseContract{
        borrowers: Mapping<AccountId, Borrow>,
        lenders: Mapping<AccountId, Lend>,
        assets: Mapping<Currency, Balance>,
        collaterals: Mapping<AccountId, Balance>,
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
                collaterals: Default::default(),
                interest_rate,
                }

        }
    
        
        #[ink(message, payable)]
        pub fn lend(&mut self, currency: Currency, amount: Balance) {
            // Gets the AccountId
            let caller = self.env().caller();

            // Panics if the amount is more or equal the account balance of caller
            assert!(amount >= self.env().balance(), "Insufficient Balance to lend!");
            
            // Gets only Lender with the AccountId
            let mut lender = self.lenders.get(&caller).unwrap();
            
            if currency == lender.currency {
                // Updates the balance 
                let previous = lender.amount;
                lender.amount = previous + amount;
            } else {
                // Creates entry 
                lender.currency = currency.clone();
                lender.amount = amount;
            }

            // Updates Pool 
            let pool_currency = self.assets.get(&currency);

            if let Some(b) =  pool_currency{
                // Updates the total 
                self.assets.insert(currency.clone(), &(b + amount));
            } else {
                // Creates entry 
                self.assets.insert(currency.clone(), &amount);
            }

        }
       
        #[ink(message)]
        pub fn borrow(&mut self, downpayment_currency: Currency, downpayment_amount: Balance, borrow_currency: Currency, borrow_amount: Balance) -> Result<()> {
            // Ensure the currency of the borrower and the lender are the same 
            if downpayment_currency != borrow_currency{
                return Err(Error::NoMatchingCurrency);
            }
            // Check if the borrower has enough funds (for implementing collateral)
            if downpayment_amount >= self.env().balance() {
                return Err(Error::InsufficientBalance);
            }
            // Gets the AccountId
            let caller = self.env().caller();

            // Gets only Borrower with the AccountId
            let mut borrower = self.borrowers.get(&caller).unwrap();
            
            if borrow_currency == borrower.currency {
                // Updates the balance 
                let previous = borrower.amount;
                borrower.amount = previous + borrow_amount;
            } else {
                // Creates entry 
                borrower.currency = borrow_currency.clone();
                borrower.amount = borrow_amount;
            }

            // Updates Pool 
            let pool_currency = self.assets.get(&borrow_currency);

            if let Some(b) =  pool_currency{
                // Updates the total 
                self.assets.insert(borrow_currency.clone(), &(b - borrow_amount));
            } else {
                // Creates entry 
                self.assets.insert(borrow_currency.clone(), &borrow_amount);
            }

            // Updates Collaterals
            let collaterals = self.collaterals.get(&caller);

            if let Some(b) =  collaterals{
                // Updates  
                self.assets.insert(downpayment_currency.clone(), &(b + downpayment_amount));
            } else {
                // Creates entry 
                self.assets.insert(downpayment_currency.clone(), &downpayment_amount);
            }

            Ok(())
        }

    }
}
