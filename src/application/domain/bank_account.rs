use uuid::Uuid;

pub struct BankAccount {
    id: String,
    /// The account balance is made up of all posted credit and debit transactions.
    /// It’s the amount you have in the account before any pending charges are added.
    account_balance: i64,
    /// Basically, an overdraft means that the bank allows customers to borrow a set amount of money.
    /// There is interest on the loan, and there is typically a fee per overdraft. 
    /// At many banks, an overdraft fee can run upwards of $35.
    overdraft_amount: i64
}

impl BankAccount {
    pub fn new(default_account_balance: i64) -> BankAccount {
        // TODO: add validation for default account balance
        BankAccount {
            id: Uuid::new_v4().to_string(),
            account_balance: default_account_balance,
            overdraft_amount: 0
        }
    }

    pub fn add_funds(&mut self, funds: i64) {
        self.account_balance += funds;
    }

    pub fn remove_funds(&mut self, amount: i64) {
        let new_account_balance = self.account_balance - amount;
        assert!(new_account_balance >= self.overdraft_amount, "Cannot ");
        self.account_balance = new_account_balance;
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_overdraft_amount(&self) -> i64 {
        self.overdraft_amount
    }

    pub fn get_account_balance(&self, ) -> i64 {
        self.account_balance
    }

}

#[cfg(test)]
mod tests {
    use super::BankAccount;

    #[test]
    fn it_should_create_an_account_wth_default_account_balance() {
        let account = BankAccount::new(100);
        assert_eq!(account.account_balance, 100);
    }

    #[test]
    fn it_should_be_able_to_add_funds() {
        let mut account: BankAccount = Account::new(100);
        account.add_funds(100);
        assert_eq!(account.account_balance, 200);
    }
}