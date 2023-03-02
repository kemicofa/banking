use crate::application::domain::transaction::Transaction;

pub trait TransactionRepository {
    fn insert(&self, transaction: &Transaction) -> Result<(), String>;
}
