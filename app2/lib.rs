#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod app2 {

    use app::AppRef;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct App2 {
        /// Stores a single `bool` value on the storage.
        value: bool,
        app: AppRef,
    }

    impl App2 {
        #[ink(constructor)]
        pub fn new(init_value: bool, app_code_hash: Hash, version: u32) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            let app = AppRef::new(init_value)
                .endowment(total_balance / 4)
                .code_hash(app_code_hash)
                .salt_bytes(salt)
                .instantiate();

            Self {
                value: init_value,
                app: app.unwrap(),
            }
        }

        // #[ink(constructor)]
        // pub fn new(init_value: bool, app_ref: AppRef) -> Self {

        //     Self {
        //         value: init_value,
        //         app: app_ref,
        //     }
        // }

        #[ink(message)]
        pub fn app_flip(&mut self) {
            self.app.flip()
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.app.get()
        }
    }
}
