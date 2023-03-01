use crate::application::domain::bank_account::BankAccount;

pub trait BankAccountRepository {
    fn insert(&self, bank_account: &BankAccount) -> Result<(), String>;
}
