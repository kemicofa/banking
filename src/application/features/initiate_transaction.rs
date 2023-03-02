use crate::application::{ports::{transaction_repository::TransactionRepository}, domain::transaction::Transaction};
use uuid::Uuid;

use super::{feature::Feature};

pub struct InitiateTransaction {
    transaction_repository: Box<dyn TransactionRepository>,
}

impl  InitiateTransaction {
    pub fn new(transaction_repository: Box<dyn TransactionRepository>) -> Self {
        Self {
            transaction_repository,
        }
    }
}

pub struct TransactionPayload {
    pub to: String,
    pub from: String,
    pub amount: i64
}

impl  Feature<TransactionPayload, Transaction> for InitiateTransaction {
    fn execute(&self, option_payload: Option<TransactionPayload>) -> Result<Transaction, String> {
        let id = Uuid::new_v4().to_string();
        let payload = option_payload.unwrap();
        let transaction = Transaction::new(
            id, 
            payload.from,
            payload.to,
            payload.amount
        );
        match self.transaction_repository.insert(&transaction) {
            Ok(()) => Ok(transaction),
            Err(err) => Err(err),
        }
    }
}
