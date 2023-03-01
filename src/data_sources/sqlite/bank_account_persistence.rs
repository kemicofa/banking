use crate::application::{ports::bank_account_repository::BankAccountRepository, dtos::bank_account_dto::BankAccountDTO, errors::Error};

use super::connector::SqliteConnector;

pub struct BankAccountPersistence {
    connector: SqliteConnector
}

impl BankAccountPersistence {
    pub fn new(connector: SqliteConnector) -> Self {
        Self {
            connector
        }
    }
}

impl BankAccountRepository for BankAccountPersistence {
    fn insert(&self, bank_account_dto: BankAccountDTO) -> Result<(), Error> {
        let result = self.connector.execute(
            "INSERT INTO person (id, account_balance) VALUES (?1, ?2)", 
            (bank_account_dto.get_id(), bank_account_dto.get_account_balance())
        );

        match result {
            Err(err) => Err(Error::new(err.to_string().as_str())),
            Ok(_) => Ok(())
        }
    }
}