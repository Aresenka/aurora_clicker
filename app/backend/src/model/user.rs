use serde::Serialize;

#[derive(Serialize)]
pub struct UserStats {
    address: String,
    balance: UserBalance,
}

impl UserStats {
    pub fn new(address: String, balance: UserBalance) -> UserStats {
        UserStats {
            address,
            balance
        }
    }
}

#[derive(Serialize)]
pub struct UserBalance {
    address: String,
    dynamic_balance: u64,
    static_balance: u64
}

impl UserBalance {
    pub fn new(address: String, dynamic_balance: u64, static_balance: u64) -> UserBalance {
        UserBalance {
            address,
            dynamic_balance,
            static_balance
        }
    }
}