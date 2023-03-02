use crate::application::domain::bank_account::BankAccount;

pub trait BankAccountRepository {
    fn insert(&self, bank_account: &BankAccount) -> Result<(), String>;
    fn query(&self) -> Result<Vec<BankAccount>, String>;
    fn get(&self, id: String) -> Result<BankAccount, String>;
    fn update(&self, bank_account: BankAccount) -> Result<(), String>;
}
