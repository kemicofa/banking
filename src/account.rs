pub struct Account {
    balance: u64
}

impl Account {
    pub fn new(default_balance: u64) -> Account {
        Account {
            balance: default_balance
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::Account;

    #[test]
    fn it_should_create_an_account_wth_default_balance() {
        let account = Account::new(0);
        assert_eq!(account.balance, 0);
    }
}