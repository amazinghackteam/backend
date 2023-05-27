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

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
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


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = LiquidZeroDogTokenRef::default();

            // When
            let contract_account_id = client
                .instantiate("LiquidZeroDogToken", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<LiquidZeroDogTokenRef>(contract_account_id.clone())
                .call(|LiquidZeroDogToken| LiquidZeroDogToken.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = LiquidZeroDogTokenRef::new(false);
            let contract_account_id = client
                .instantiate("LiquidZeroDogToken", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<LiquidZeroDogTokenRef>(contract_account_id.clone())
                .call(|LiquidZeroDogToken| LiquidZeroDogToken.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<LiquidZeroDogTokenRef>(contract_account_id.clone())
                .call(|LiquidZeroDogToken| LiquidZeroDogToken.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<LiquidZeroDogTokenRef>(contract_account_id.clone())
                .call(|LiquidZeroDogToken| LiquidZeroDogToken.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
