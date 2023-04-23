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
        USDT,
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
        interest_rate: Balance,
        interest_currency: Currency,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Borrow {
        amount: Balance,
        currency: Currency, 
        collateral: Balance,
        collateral_currency: Currency,
        interest_rate: u32,
        interest_currency: Currency
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct BitleaseContract{
        borrowers: Mapping<AccountId, Borrow>,
        lenders: Mapping<AccountId, Lend>,
        assets: Mapping<Currency, Balance>,
    }

    /// Specify the result type.
    pub type Result<T> = core::result::Result<T, Error>;


    impl BitleaseContract{
        /// Constructor that initializes the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                borrowers: Default::default(),
                lenders: Default::default(),
                assets: Default::default(),
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
                let new_lend = Lend{
                    amount: amount,
                    currency: currency.clone(),
                    interest_rate: 10,
                    interest_currency: currency.clone(),
                };
                self.lenders.insert(caller, &new_lend);
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
                // Updates the balance and collateral 
                let previous_amount = borrower.amount;
                borrower.amount = previous_amount + borrow_amount;
                let previous_collateral = borrower.collateral;
                borrower.collateral = previous_collateral + downpayment_amount;
            } else {
                // Creates Borrow 
                let new_borrow = Borrow{
                    amount: borrow_amount,
                    currency: borrow_currency.clone(),
                    collateral: downpayment_amount,
                    collateral_currency: downpayment_currency.clone(),
                    interest_rate: 10,
                    interest_currency: borrow_currency.clone(),
                };
                self.borrowers.insert(caller, &new_borrow);
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

            Ok(())
        }

    }
}
