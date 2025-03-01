use std::rc::Rc;

use crate::application::{
    domain::transaction::Transaction,
    ports::{
        bank_account_repository::BankAccountRepository,
        transaction_repository::TransactionRepository,
    },
};
use uuid::Uuid;

use super::feature::Feature;

pub struct InitiateTransaction {
    bank_account_repository: Rc<dyn BankAccountRepository>,
    transaction_repository: Rc<dyn TransactionRepository>,
}

impl InitiateTransaction {
    pub fn new(
        bank_account_repository: Rc<dyn BankAccountRepository>,
        transaction_repository: Rc<dyn TransactionRepository>,
    ) -> Self {
        Self {
            bank_account_repository,
            transaction_repository,
        }
    }
}

pub struct TransactionPayload {
    pub to: String,
    pub from: String,
    pub amount: u64,
}

impl Feature<TransactionPayload, Transaction> for InitiateTransaction {
    fn execute(&self, payload: TransactionPayload) -> Result<Transaction, String> {
        let id = Uuid::new_v4().to_string();
        let transaction =
            Transaction::new(id, payload.from.clone(), payload.to.clone(), payload.amount);
        let mut bank_account_from = self
            .bank_account_repository
            .get(payload.from.clone())
            .unwrap();
        let mut bank_account_to = self
            .bank_account_repository
            .get(payload.to.clone())
            .unwrap();

        bank_account_from.apply_transaction(&transaction);
        bank_account_to.apply_transaction(&transaction);

        self.bank_account_repository
            .update(bank_account_from)
            .unwrap();
        self.bank_account_repository
            .update(bank_account_to)
            .unwrap();

        match self.transaction_repository.insert(&transaction) {
            Ok(()) => Ok(transaction),
            Err(err) => Err(err),
        }
    }
}
