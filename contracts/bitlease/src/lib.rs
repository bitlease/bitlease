#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bitlease_contract {
    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Currency {
        ASTAR,
        USDT,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if Currency doesn't match
        NoMatchingCurrency,
        /// Returned if not enough balance
        InsufficientBalance,
        /// Returned if not a Lender
        NotALender,
        /// Returned if not a Borrower
        NotABorrower,
        /// Returned if the transfer failed
        TransferFailed,
    }

    #[derive(scale::Decode, scale::Encode, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Lend {
        amount: Balance,
        interest_rate: u32,
        interest_currency: Currency,
    }

    #[derive(scale::Decode, scale::Encode, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Borrow {
        amount: Balance,
        collateral: Balance,
        collateral_currency: Currency,
        interest_rate: u32,
        interest_currency: Currency,
        start: Option<Timestamp>,
        close: Option<Timestamp>,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct BitleaseContract {
        borrowers: Mapping<(AccountId, Currency), Borrow>,
        lenders: Mapping<(AccountId, Currency), Lend>,
        assets: Mapping<Currency, Balance>,
        interests: Mapping<Currency, Balance>,
    }

    /// Specify the result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl BitleaseContract {
        /// Constructor that initializes the contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                borrowers: Default::default(),
                lenders: Default::default(),
                assets: Default::default(),
                interests: Default::default(),
            }
        }

        #[ink(message, payable)]
        pub fn lend(&mut self, currency: Currency) {
            // Gets the AccountId
            let caller = self.env().caller();

            // Defines the amount been transfered
            let amount = self.env().transferred_value();

            // Gets only Lender with the AccountId and currency
            let lender = self.lenders.get(&(caller, currency.clone()));
            if let Some(b) = lender {
                let new_lend = Lend {
                    amount: b.amount + amount,
                    interest_rate: b.interest_rate,
                    interest_currency: b.interest_currency,
                };
                self.lenders.insert(&(caller, currency.clone()), &new_lend);
            } else {
                let new_lend = Lend {
                    amount: amount,
                    interest_rate: 5,
                    interest_currency: currency.clone(),
                };
                self.lenders.insert(&(caller, currency.clone()), &new_lend);
            }

            // Updates Pool
            let pool_currency = self.assets.get(&currency);

            if let Some(b) = pool_currency {
                // Updates the total
                self.assets.insert(currency.clone(), &(b + amount));
            } else {
                self.assets.insert(currency.clone(), &amount);
            }
        }

        #[ink(message, payable)]
        pub fn borrow(
            &mut self,
            downpayment_currency: Currency,
            borrow_currency: Currency,
            borrow_amount: Balance,
        ) -> Result<()> {
            // Ensure the currency of the borrower and the lender are the same
            if downpayment_currency != borrow_currency {
                return Err(Error::NoMatchingCurrency);
            }
            // Gets the AccountId
            let caller = self.env().caller();

            // Defines the amount been transfered
            let downpayment_amount = self.env().transferred_value();

            // Gets only Borrower with the AccountId and currency
            let borrower = self.borrowers.get(&(caller, downpayment_currency.clone()));
            if let Some(b) = borrower {
                let new_borrow = Borrow {
                    amount: b.amount + borrow_amount,
                    collateral: b.collateral + downpayment_amount,
                    collateral_currency: b.collateral_currency,
                    interest_rate: b.interest_rate,
                    interest_currency: b.interest_currency,
                    start: b.start,
                    close: b.close,
                };
                self.borrowers
                    .insert(&(caller, downpayment_currency.clone()), &new_borrow);
            } else {
                let new_borrow = Borrow {
                    amount: borrow_amount,
                    collateral: downpayment_amount,
                    collateral_currency: downpayment_currency.clone(),
                    interest_rate: 5,
                    interest_currency: borrow_currency.clone(),
                    start: Some(self.env().block_timestamp()),
                    close: None,
                };
                self.borrowers
                    .insert(&(caller, downpayment_currency.clone()), &new_borrow);
            }
            // Updates Pool
            let pool_currency = self.assets.get(&borrow_currency);

            if let Some(b) = pool_currency {
                // Updates the total
                self.assets
                    .insert(borrow_currency.clone(), &(b - borrow_amount));
            } else {
                // Creates entry
                self.assets.insert(borrow_currency.clone(), &borrow_amount);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn getter_lender(&self, currency: Currency) -> Option<Balance> {
            // Gets the AccountId
            let caller = self.env().caller();
            // If the caller is lender
            if self.lenders.get(&(caller, currency.clone())) != None {
                // Gets the amount with the AccountId provided
                let lender = self.lenders.get(&(caller, currency.clone())).unwrap();
                let amount = lender.amount;
                return Some(amount);
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn getter_borrower(&self, currency: Currency) -> Option<Balance> {
            // Gets the AccountId
            let caller = self.env().caller();
            // If the caller is borrower
            if self.borrowers.get(&(caller, currency.clone())) != None {
                // Gets the amount with the AccountId provided
                let borrower = self.borrowers.get(&(caller, currency.clone())).unwrap();
                let amount = borrower.amount;
                return Some(amount);
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self, currency: Currency, amount: Balance) -> Result<()> {
            // Gets the AccountId
            let caller = self.env().caller();
            // Gets only Lender with the AccountId
            let lender = self.lenders.get(&(caller, currency.clone()));
            if let Some(b) = lender {
                if b.amount < amount {
                    return Err(Error::InsufficientBalance);
                } else {
                    // Updates the balance in lender
                    let new_lend = Lend {
                        amount: b.amount - amount,
                        interest_rate: b.interest_rate,
                        interest_currency: b.interest_currency,
                    };
                    // Updates Pool
                    let pool_currency = self.assets.get(&currency);
                    if let Some(b) = pool_currency {
                        // Updates the total
                        self.assets.insert(currency.clone(), &(b - amount));
                    }
                    self.lenders.insert(&(caller, currency.clone()), &new_lend);
                    self.env()
                        .transfer(caller, amount)
                        .map_err(|_| Error::TransferFailed)?;
                    Ok(())
                }
            } else {
                return Err(Error::NotALender);
            }
        }

        #[ink(message, payable)]
        pub fn pay_interest(&mut self, currency: Currency) -> Result<()> {
            // Checks caller is borrower
            let caller = self.env().caller();

            // Defines the amount been transfered
            let amount = self.env().transferred_value();

            let borrower = self.borrowers.get(&(caller, currency.clone()));
            if let Some(b) = borrower {
                // Checks amount of tokens transferred is equal to interest amount
                assert_eq!(amount, ((b.amount * b.interest_rate as u128) / 100) as u128);
                // Updates interests pool
                let interests = self.interests.get(&currency.clone()).unwrap();
                let new = amount + interests;
                self.interests.insert(&currency.clone(), &new);
                Ok(())
            } else {
                return Err(Error::NotABorrower);
            }
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
    fn lend_works() {
        let alice = alice();
        ink::env::test::set_account_balance::<Environment>(alice, 2000);
        ink::env::test::set_caller::<Environment>(alice);
        let mut contract = BitleaseContract::new();
        let currency = Currency::USDT;
        contract.lend(currency.clone(), 100);
        ink::env::test::transfer_in::<Environment>(100);
        assert_eq!(contract.get_deposit(currency.clone()).unwrap(), 100);
    }

    #[ink::test]
    fn lend_works2() {
        let alice = alice();
        ink::env::test::set_account_balance::<Environment>(alice, 2000);
        ink::env::test::set_caller::<Environment>(alice);
        let mut contract = BitleaseContract::new();
        let currency = Currency::USDT;
        contract.lend(currency.clone(), 300);
        ink::env::test::transfer_in::<Environment>(300);
        assert_eq!(contract.get_deposit(currency.clone()).unwrap(), 300);
    }

    #[ink::test]
    fn borrow_works() {
        let bob = bob();
        ink::env::test::set_account_balance::<Environment>(bob, 2000);
        ink::env::test::set_caller::<Environment>(bob);
        let mut contract = BitleaseContract::new();
        let downpayment_currency = Currency::USDT;
        let borrow_currency = Currency::USDT;
        contract.borrow(downpayment_currency.clone(), 1000, borrow_currency, 3000);
        assert_eq!(
            contract.get_deposit(downpayment_currency.clone()).unwrap(),
            3000
        );
    }

    #[ink::test]
    fn withdraw_works() {
        let alice = alice();
        ink::env::test::set_account_balance::<Environment>(alice, 2000);
        ink::env::test::set_caller::<Environment>(alice);
        let mut contract = BitleaseContract::new();
        let currency = Currency::USDT;
        contract.lend(currency.clone(), 100);
        contract.withdraw(currency.clone(), 20);
        assert_eq!(contract.get_deposit(currency.clone()).unwrap(), 80);
    }
}
