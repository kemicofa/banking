use crate::application::{
    domain::bank_account::BankAccount, ports::bank_account_repository::BankAccountRepository,
};

use super::connector::SqliteConnector;

pub struct BankAccountPersistence {
    connector: Box<SqliteConnector>,
}

impl BankAccountPersistence {
    pub fn new(connector: Box<SqliteConnector>) -> Self {
        Self { connector }
    }
}

impl BankAccountRepository for BankAccountPersistence {
    fn insert(&self, bank_account: &BankAccount) -> Result<(), String> {
        let result = self.connector.execute(
            "INSERT INTO bankaccounts (id, fullname, account_balance) VALUES (?1, ?2, ?3)",
            (
                bank_account.id.clone(),
                bank_account.fullname.clone(),
                bank_account.account_balance,
            ),
        );

        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => Ok(()),
        }
    }

    fn query(&self) -> Result<Vec<BankAccount>, String> {
        let stmt = self
            .connector
            .prepare("SELECT id, fullname, account_balance FROM bankaccounts");
        let mut binding = stmt.unwrap();
        let bank_accounts_iter = binding
            .query_map([], |row| {
                Ok(BankAccount::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                ))
            })
            .unwrap();

        let bank_accounts = bank_accounts_iter
            .map(|option_bank_account| option_bank_account.unwrap())
            .collect();

        Ok(bank_accounts)
    }

    fn get(&self, id: String) -> Result<BankAccount, String> {
        let stmt = self
            .connector
            .prepare("SELECT id, fullname, account_balance FROM bankaccounts WHERE id = :id;");
        let mut binding = stmt.unwrap();
        let bank_accounts_iter = binding
            .query_map(&[(":id", id.as_str())], |row| {
                Ok(BankAccount::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                ))
            })
            .unwrap();

        let bank_account: Vec<BankAccount> = bank_accounts_iter
            .map(|option_bank_account| option_bank_account.unwrap())
            .collect();

        Ok(bank_account[0].clone())
    }

    fn update(&self, bank_account: BankAccount) -> Result<(), String> {
        let result = self.connector.execute(
            "UPDATE bankaccounts SET fullname = ?1, account_balance = ?2 WHERE id = ?3",
            (
                bank_account.fullname.clone(),
                bank_account.account_balance,
                bank_account.id.clone(),
            ),
        );

        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => Ok(()),
        }
    }
}
