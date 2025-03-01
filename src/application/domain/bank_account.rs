use std::fmt;

use serde::{Deserialize, Serialize};

use super::transaction::{Transaction};

/// A bank account is a financial account provided by a bank or other financial institution to an
/// individual or business entity that allows them to deposit, withdraw, and manage money. Bank
/// accounts can be used for various purposes such as storing money, receiving and making payments,
/// and earning interest on deposits.
#[derive(Serialize, Deserialize, Clone)]
pub struct BankAccount {
    pub id: String,
    pub fullname: String,
    /// The account balance is made up of all posted credit and debit transactions.
    /// It’s the amount you have in the account before any pending charges are added.
    pub account_balance: u64,
}

impl BankAccount {
    /// Returns a bank account
    ///
    /// # Arguments
    /// * `id` - A string representing the unique id of the bank account
    /// * `fullname`- Fullname of the owner of the bank account
    /// * `default_account_balance` - initial amount of the bank account. Divide by 100 to find the amount in euros.
    ///
    /// # Examples
    ///
    /// ```
    /// let bank_account = BankAccount::new("my-unique-id", "Kevin", 0);
    /// ```
    ///
    pub fn new(id: String, fullname: String, default_account_balance: u64) -> BankAccount {
        // TODO: add validation for default account balance
        BankAccount {
            id,
            fullname,
            account_balance: default_account_balance,
        }
    }

    pub fn apply_transaction(&mut self, transaction: &Transaction) -> Result<(), &str> {
        if self.id == transaction.from {
            self.account_balance -= transaction.amount;
            return Ok(());
        }

        if self.id == transaction.to {
            self.account_balance += transaction.amount;
            return Ok(());
        }

        Err("Could not find matching bank account id")
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
}
