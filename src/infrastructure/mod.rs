use crate::{
    application::{
        features::{
            feature::Feature,
            open_bank_account::OpenBankAccount,
        },
        ports::bank_account_repository::BankAccountRepository,
    },
    data_sources::sqlite::{
        bank_account_persistence::BankAccountPersistence, connector::SqliteConnector,
    },
};

pub struct Container {
    pub open_bank_account: Box<OpenBankAccount>,
}

impl Container {
    pub fn new() -> Self {
        let sqlite_connector = Box::new(SqliteConnector::new());
        let bank_account_repository = Box::new(BankAccountPersistence::new(sqlite_connector))
            as Box<dyn BankAccountRepository + 'static>;
        let open_bank_account = Box::new(OpenBankAccount::new(bank_account_repository));
        Self { open_bank_account }
    }
}
