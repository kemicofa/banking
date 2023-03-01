use std::collections::HashMap;
use core::any::Any;
use crate::{application::{features::{feature::Feature, open_bank_account::{OpenBankAccount, self}}, ports::bank_account_repository::{self, BankAccountRepository}}, data_sources::sqlite::{bank_account_persistence::BankAccountPersistence, connector::SqliteConnector}};

pub struct Container {
    pub open_bank_account: OpenBankAccount
}

impl Container {
    pub fn new() -> Self {
        let sqlite_connector = SqliteConnector::new();
        let bank_account_repository = BankAccountPersistence::new(sqlite_connector);
        let open_bank_account = OpenBankAccount::new(&bank_account_repository);
        Self {
            open_bank_account
        }
    }
}
