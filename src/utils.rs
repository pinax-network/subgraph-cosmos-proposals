use crate::pb::cosmos::base::v1beta1::Coin;

pub fn extract_initial_deposit(initial_deposit: &[Coin]) -> (&str, &str) {
    initial_deposit
        .get(0)
        .map_or(("", "0"), |deposit| (deposit.denom.as_str(), deposit.amount.as_str()))
}
