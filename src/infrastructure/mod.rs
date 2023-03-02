use crate::{
    application::{
        features::{
            open_bank_account::OpenBankAccount,
        },
        ports::bank_account_repository::{BankAccountRepository, self},
    },
    data_sources::sqlite::{
        bank_account_persistence::BankAccountPersistence, connector::SqliteConnector,
    },
};

pub struct Container {
    pub open_bank_account: Box<OpenBankAccount>,
    pub bank_account_repository: Box<dyn BankAccountRepository>
}

fn build_bank_account_repository() -> impl BankAccountRepository {
    let sqlite_connector = Box::new(SqliteConnector::new());
    BankAccountPersistence::new(sqlite_connector)
}

impl  Container {
    pub fn new() -> Self {
        let bank_account_repository_1 = build_bank_account_repository();
        let bank_account_repository_2 = build_bank_account_repository();

        let open_bank_account = OpenBankAccount::new(Box::new(bank_account_repository_1));
        Self { 
            open_bank_account: Box::new(open_bank_account),
            bank_account_repository: Box::new(bank_account_repository_2)
        }
    }
}

