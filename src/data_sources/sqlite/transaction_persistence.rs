use crate::application::{
    domain::transaction::Transaction, ports::transaction_repository::TransactionRepository,
};

use super::connector::SqliteConnector;

pub struct TransactionPersistence {
    connector: Box<SqliteConnector>,
}

impl TransactionPersistence {
    pub fn new(connector: Box<SqliteConnector>) -> Self {
        Self { connector }
    }
}

impl TransactionRepository for TransactionPersistence {
    fn insert(&self, transaction: &Transaction) -> Result<(), String> {
        let result = self.connector.execute(
            "INSERT INTO transactions (id, _from, _to, amount) VALUES (?1, ?2, ?3, ?4)",
            (
                transaction.id.clone(),
                transaction.from.clone(),
                transaction.to.clone(),
                transaction.amount,
            ),
        );

        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => Ok(()),
        }
    }
}
