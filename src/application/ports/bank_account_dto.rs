use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BankAccountDTO {
    id: String,
    account_balance: i64,
    overdraft_amount: i64
}

impl BankAccountDTO {
    pub fn new(id: String, account_balance: i64, overdraft_amount: i64) -> BankAccountDTO {
        BankAccountDTO {
            id,
            account_balance,
            overdraft_amount
        }
    }

    pub fn getId(&self) -> String {
        self.id.clone()
    }
}