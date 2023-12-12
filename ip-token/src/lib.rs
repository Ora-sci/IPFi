pub use core_impl::FungibleToken;
pub use resolver::FungibleTokenResolver;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

pub mod core;
pub mod core_impl;
pub mod internal;
pub mod ip_token;
pub mod metadata;
pub mod receiver;
pub mod resolver;
pub mod storage_impl;
pub mod storage_management;

use crate::metadata::FungibleTokenMetadata;

use crate::internal::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: FungibleTokenMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            owner_id: owner_id.clone(),
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token
            .internal_register_account(&metadata.original_owner);
        //this.token.internal_deposit(account_id, amount)

        this
    }
}
