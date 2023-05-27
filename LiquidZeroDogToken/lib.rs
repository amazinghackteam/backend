#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod LiquidZeroDogToken {
    use ink::storage::Mapping;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct LiquidZeroDogToken {
        /// Stores a single `bool` value on the storage.
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl LiquidZeroDogToken {
        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
       #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self {
                total_supply,
                balances,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn total_supply_works() {
            let LiquidZeroDogToken = LiquidZeroDogToken::new(100);
            assert_eq!(LiquidZeroDogToken.total_supply(), 100);
        }

        #[ink::test]
        fn balance_of_works() {
            let LiquidZeroDogToken = LiquidZeroDogToken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(LiquidZeroDogToken.balance_of(accounts.alice), 100);
            assert_eq!(LiquidZeroDogToken.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut LiquidZeroDogToken = LiquidZeroDogToken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(LiquidZeroDogToken.balance_of(accounts.bob), 0);
            assert_eq!(LiquidZeroDogToken.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(LiquidZeroDogToken.balance_of(accounts.bob), 10);
        }

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let LiquidZeroDogToken = LiquidZeroDogToken::default();
            assert_eq!(LiquidZeroDogToken.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut LiquidZeroDogToken = LiquidZeroDogToken::new(false);
            assert_eq!(LiquidZeroDogToken.get(), false);
            LiquidZeroDogToken.flip();
            assert_eq!(LiquidZeroDogToken.get(), true);
        }
    }
}
