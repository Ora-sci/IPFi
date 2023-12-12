use crate::*;

use crate::core_impl::AccessType;
use near_sdk::{env, near_bindgen, AccountId, Balance};

#[near_bindgen]
impl Contract {
    // Isssue ip token based on ip nft
    pub fn issue_ip_token(&mut self, receiver: AccountId, access_type: AccessType) {

        let issuer = env::signer_account_id();
        assert!(self.is_owner_or_issuer(issuer), "Not owner or issuer");
        self.token.internal_deposit(&receiver, 1);
        self.token.internal_grant_access(&receiver, access_type)
    }

    // Get issuer
    pub fn get_issuer(&self) -> AccountId {
        if let Some(meta) = self.metadata.get() {
            meta.original_owner
        } else {
            env::panic_str("Metadata is not existing");
        }
    }


    // Get token access
    pub fn get_token_access(&self, token: Balance) -> AccessType {
        match self.token.permissions.get(&token) {
            Some(access_type) => return access_type,
            None => env::panic_str("Token not found"),
        }
    }

    pub fn get_metadata(&self) -> FungibleTokenMetadata {
        if let Some(meta) = self.metadata.get() {
            meta
        } else {
            env::panic_str("Metadata is not existing");
        }
    }

}

// Helper function for ip token
impl Contract {

    pub(crate) fn is_owner_or_issuer(&self, account_id: AccountId) -> bool {
        if let Some(meta) = self.metadata.get() {
            account_id == self.owner_id || meta.original_owner == account_id
        } else {
            env::panic_str("Metadata isnt existed ");
        }
    }
}
