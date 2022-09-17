pub struct Account {
    /// The available balance is the amount one can use for purchases or withdrawals.
    /// It's made up of the account balance minus pending debit card authorizations and holds on funds.
    available_balance: i64,
    /// The account balance is made up of all posted credit and debit transactions.
    /// It’s the amount you have in the account before any pending charges are added.
    account_balance: i64
}

impl Account {
    pub fn new(default_account_balance: i64) -> Account {
        Account {
            available_balance: default_account_balance,
            account_balance: default_account_balance
        }
    }

    pub fn get_available_balance(&self) -> i64 {
        self.available_balance
    }

    pub fn get_account_balance(&self) -> i64 {
        self.account_balance
    }
}

#[cfg(test)]
mod tests {
    use super::Account;

    #[test]
    fn it_should_create_an_account_wth_default_account_balance() {
        let account = Account::new(100);
        assert_eq!(account.account_balance, 100);
    }

    #[test]
    fn it_should_create_an_account_with_matching_available_balance() {
        let account = Account::new(100);
        assert_eq!(account.available_balance, 100);
    }
}