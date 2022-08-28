use crate::model::user::{
    UserBalance,
    UserStats
};
use std::collections::HashMap;

pub fn get_user_balance_by_address(address: &str) -> Option<UserBalance>
{
    //TODO: implement balance fetching
    let mut balances_map: HashMap<&str, UserBalance> = HashMap::new();
    let more_dynamic_balance: UserBalance = UserBalance::new(
        "moredynamic".to_string(),
        1488,
        39
    );
    let more_static_balance: UserBalance = UserBalance::new(
        "morestatic".to_string(),
        42,
        9000
    );

    balances_map.insert("moredynamic", more_dynamic_balance);
    balances_map.insert("morestatic", more_static_balance);

    balances_map.remove(address)
}

pub fn get_user_stats_by_address(address: String) -> Option<UserStats> {
    //TODO: implement stats fetching
    let balance: Option<UserBalance> = get_user_balance_by_address(address.as_str());

    if balance.is_none() {
        None
    } else {
        Some(UserStats::new(address, balance.unwrap()))
    }
}