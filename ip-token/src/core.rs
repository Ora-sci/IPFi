use near_sdk::ext_contract;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk::PromiseOrValue;

#[ext_contract(ext_ft_core)]
pub trait FungibleTokenCore {

    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_total_supply(&self) -> U128;

    fn ft_balance_of(&self, account_id: AccountId) -> U128;

}