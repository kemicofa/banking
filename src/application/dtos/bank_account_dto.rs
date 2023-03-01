use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BankAccountDTO {
    id: String,
    account_balance: i64,
    overdraft_amount: i64,
}

impl BankAccountDTO {
    pub fn new(id: String, account_balance: i64, overdraft_amount: i64) -> Self {
        Self {
            id,
            account_balance,
            overdraft_amount,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_account_balance(&self) -> i64 {
        self.account_balance
    }

    pub fn to_string(&self) -> String {
        r#"
            Account id: :id
            Account balance: :account_balance
            Overdraft amount: :overdraft_amount
        "#.
        to_string()
        .replace(":id", &self.id)
        .replace(":account_balance", &self.account_balance.to_string())
        .replace(":overdraft_amount", &self.overdraft_amount.to_string())
    }
}
