use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BankAccount {
    pub id: String,
    pub fullname: String,
    /// The account balance is made up of all posted credit and debit transactions.
    /// It’s the amount you have in the account before any pending charges are added.
    pub account_balance: i64,
}

impl BankAccount {
    pub fn new(id: String, fullname: String, default_account_balance: i64) -> BankAccount {
        // TODO: add validation for default account balance
        BankAccount {
            id,
            fullname,
            account_balance: default_account_balance,
        }
    }

    pub fn add_funds(&mut self, funds: i64) {
        self.account_balance += funds;
    }

    pub fn remove_funds(&mut self, amount: i64) {
        let new_account_balance = self.account_balance - amount;
        self.account_balance = new_account_balance;
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "> {}'s bank account ({}) with balance {:.2} €",
            self.fullname,
            self.id,
            (self.account_balance as f64) / 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::BankAccount;
    const FULLNAME: &'static str = "Kevin Faulhaber";
    #[test]
    fn it_should_create_an_account_wth_default_account_balance() {
        let account =
            BankAccount::new("__BANK_ACCOUNT_ID__".to_string(), FULLNAME.to_string(), 100);
        assert_eq!(account.account_balance, 100);
    }

    #[test]
    fn it_should_be_able_to_add_funds() {
        let mut account =
            BankAccount::new("__BANK_ACOUNT_ID__".to_string(), FULLNAME.to_string(), 100);
        account.add_funds(100);
        assert_eq!(account.account_balance, 200);
    }

    #[test]
    fn it_should_be_able_to_remove_funds() {
        let mut account =
            BankAccount::new("__BANK_ACOUNT_ID__".to_string(), FULLNAME.to_string(), 100);
        account.remove_funds(100);
        assert_eq!(account.account_balance, 0);
    }
}
