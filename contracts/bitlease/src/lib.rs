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
        /// Returned if not a Lender
        NotALender,
        UnexpectedError, 
    }

    #[derive(scale::Decode, scale::Encode, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(
            scale_info::TypeInfo, 
            ink::storage::traits::StorageLayout)
    )]
    pub struct Lend {
        amount: Balance,
        currency: Currency,
        interest_rate: Balance,
        interest_currency: Currency,
    }

    #[derive(scale::Decode, scale::Encode, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(
            scale_info::TypeInfo, 
            ink::storage::traits::StorageLayout)
    )]
    pub struct Borrow {
        amount: Balance,
        currency: Currency, 
        collateral: Balance,
        collateral_currency: Currency,
        interest_rate: u32,
        interest_currency: Currency,
        start: Option<Timestamp>,
        close: Option<Timestamp>,
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

            // Gets only Lender with the AccountId
            let lender = self.lenders.get(&caller);
            if let Some(mut b) =  lender {
                if currency == b.currency {
                    // Updates the balance 
                    let previous = b.amount;
                    b.amount = previous + amount;
                } else {
                    b.currency = currency.clone();
                    b.amount = amount;
                }
            } else {
                let new_lend = Lend{
                    amount: amount,
                    currency: currency.clone(),
                    interest_rate: 10,
                    interest_currency: currency.clone(),
                };
                self.lenders.insert(caller, &new_lend);
            }

            // Updates Pool 
            let pool_currency = self.assets.get(&currency);

            if let Some(b) =  pool_currency{
                // Updates the total 
                self.assets.insert(currency.clone(), &(b + amount));
            } else {
                self.assets.insert(currency.clone(), &amount);
            }

        }
       
        #[ink(message)]
        pub fn borrow(&mut self, downpayment_currency: Currency, downpayment_amount: Balance, borrow_currency: Currency, borrow_amount: Balance) -> Result<()> {
            // Ensure the currency of the borrower and the lender are the same 
            if downpayment_currency != borrow_currency{
                return Err(Error::NoMatchingCurrency);
            }
            // Gets the AccountId
            let caller = self.env().caller();

            // Gets only Borrower with the AccountId
            let borrower = self.borrowers.get(&caller);
            if let Some(mut b) =  borrower {
                if borrow_currency == b.currency {
                    // Updates the balance 
                    let previous_amount = b.amount;
                    b.amount = previous_amount + borrow_amount;
                    let previous_collateral = b.collateral;
                    b.collateral = previous_collateral + downpayment_amount;
                    b.start = Some(self.env().block_timestamp());
                } 
            } else {
                let new_borrow = Borrow{
                    amount: borrow_amount,
                    currency: borrow_currency.clone(),
                    collateral: downpayment_amount,
                    collateral_currency: downpayment_currency.clone(),
                    interest_rate: 10,
                    interest_currency: borrow_currency.clone(),
                    start: Some(self.env().block_timestamp()),
                    close: None,
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

        #[ink(message)]
        pub fn get_deposit(&self) -> Option<Balance> {
            // Gets the AccountId
            let caller = self.env().caller();
            // If the caller is lender 
            if self.lenders.get(&caller) != None {
                // Gets the lender with the AccountId provided
                let lender = self.lenders.get(&caller).unwrap();
                let amount = lender.amount;
                return Some(amount);
            } else if self.borrowers.get(&caller) != None {
                // If the caller is borrower 
                let borrower = self.borrowers.get(&caller). unwrap();
                let amount = borrower.amount;
                return Some(amount);
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self, currency: Currency, amount: Balance) -> Result<()>{
            // Gets the AccountId 
            let caller = self.env().caller();
            // Gets only Lender with the AccountId
            let lender = self.lenders.get(&caller);
            if let Some(mut b) = lender {
                if b.currency != currency{
                    return Err(Error::NoMatchingCurrency);
                } else {
                    if b.amount < amount {
                        return Err(Error::InsufficientBalance)
                    } else {
                        let amount_transfer = b.amount;
                        // Updates the balance in lender
                        b.amount = b.amount - amount;
                        ink::env::transfer::<Environment>(caller, amount_transfer).map_err(|_| Error::UnexpectedError)
                    }
                }    
            } else {
                return Err(Error::NotALender)
            }
        }
    }

    
    #[cfg(test)]
    mod tests {
        use super::*;

        // We define some helper Accounts to make our tests more readable
        fn default_accounts() -> ink::env::test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn alice() -> AccountId {
            default_accounts().alice
        }

        fn bob() -> AccountId {
            default_accounts().bob
        }

        #[ink::test]
        fn lend_works(){
            let alice = alice();
            ink::env::test::set_account_balance::<Environment>(alice, 2000);
            ink::env::test::set_caller::<Environment>(alice);
            let mut contract = BitleaseContract::new();
            let currency = Currency::USDT;
            contract.lend(currency, 100);
            assert_eq!(contract.get_deposit().unwrap(), 100);
        }

        #[ink::test]
        fn borrow_works(){
            let bob = bob();
            ink::env::test::set_account_balance::<Environment>(bob, 2000);
            ink::env::test::set_caller::<Environment>(bob);
            let mut contract = BitleaseContract::new();
            let downpayment_currency = Currency::USDT;
            let borrow_currency = Currency::USDT;
            contract.borrow(downpayment_currency, 1000, borrow_currency, 3000);
            assert_eq!(contract.get_deposit().unwrap(), 3000);
        }
    }
}