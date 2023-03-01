use std::collections::HashMap;
use core::any::Any;
use crate::{application::{features::{feature::Feature, open_bank_account::{OpenBankAccount, self}}, ports::bank_account_repository::{self, BankAccountRepository}}, data_sources::sqlite::{bank_account_persistence::BankAccountPersistence, connector::SqliteConnector}};

use self::consts::OPEN_BANK_ACCOUNT;

pub mod consts;

type AnyFeature = dyn Feature<dyn Any, dyn Any>;

pub struct Container {
    features: HashMap<String, Box<AnyFeature>>
}

impl Container {
    pub fn new() -> Self {
        let sqlite_connector = SqliteConnector::new();
        let bank_account_repository = BankAccountPersistence::new(sqlite_connector);
        let open_bank_account = OpenBankAccount::new(&bank_account_repository);
        Self {
            features: HashMap::from([
                (OPEN_BANK_ACCOUNT.to_string(), Box::new(open_bank_account))
            ])
        }
    }

    fn get_feature(&mut self, key: &'static str) -> AnyFeature {
        match self.features.get(key) {
            Some(feature) => feature,
            None => panic!("Requested feature did not exist.")
        }
    }
}
