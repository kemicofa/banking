use crate::{
    application::{
        features::{
            open_bank_account::OpenBankAccount, initiate_transaction::InitiateTransaction,
        },
        ports::{bank_account_repository::{BankAccountRepository, self}, transaction_repository::TransactionRepository},
    },
    data_sources::sqlite::{
        bank_account_persistence::BankAccountPersistence, connector::SqliteConnector, transaction_persistence::TransactionPersistence,
    },
};

pub struct Container {
    pub open_bank_account: Box<OpenBankAccount>,
    pub bank_account_repository: Box<dyn BankAccountRepository>,
    pub initiate_transaction: Box<InitiateTransaction>
}

fn build_bank_account_repository() -> impl BankAccountRepository {
    let sqlite_connector = Box::new(SqliteConnector::new());
    BankAccountPersistence::new(sqlite_connector)
}

fn build_transaction_repository() -> impl TransactionRepository {
    let sqlite_connector = Box::new(SqliteConnector::new());
    TransactionPersistence::new(sqlite_connector)
}

impl  Container {
    pub fn new() -> Self {
        let bank_account_repository_1 = build_bank_account_repository();
        let bank_account_repository_2 = build_bank_account_repository();
        let bank_account_repository_3 = build_bank_account_repository();
        let transaction_repository = build_transaction_repository();
        let initiate_transaction = InitiateTransaction::new(Box::new(bank_account_repository_3), Box::new(transaction_repository));
        let open_bank_account = OpenBankAccount::new(Box::new(bank_account_repository_1));
        Self { 
            open_bank_account: Box::new(open_bank_account),
            bank_account_repository: Box::new(bank_account_repository_2),
            initiate_transaction: Box::new(initiate_transaction)
        }
    }
}
