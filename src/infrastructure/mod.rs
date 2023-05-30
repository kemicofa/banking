use std::rc::Rc;

use crate::application::{features::{open_bank_account::OpenBankAccount, initiate_transaction::InitiateTransaction}, ports::{bank_account_repository::BankAccountRepository, transaction_repository::TransactionRepository}};

use self::data_sources::sqlite::{connector::SqliteConnector, transaction_persistence::TransactionPersistence, bank_account_persistence::BankAccountPersistence};

pub mod transport_layer;
pub mod data_sources;

pub struct Container {
    pub open_bank_account: OpenBankAccount,
    pub bank_account_repository: Rc<dyn BankAccountRepository>,
    pub transaction_repository: Rc<dyn TransactionRepository>,
    pub initiate_transaction: InitiateTransaction,
}

impl Container {
    pub fn new() -> Self {
        let rc_sqlite_connector = Rc::new(SqliteConnector::new());
        let rc_transaction_repository: Rc<dyn TransactionRepository> =
            Rc::new(TransactionPersistence::new(Rc::clone(&rc_sqlite_connector)));
        let rc_bank_account_repository: Rc<dyn BankAccountRepository> =
            Rc::new(BankAccountPersistence::new(Rc::clone(&rc_sqlite_connector)));

        let initiate_transaction = InitiateTransaction::new(
            Rc::clone(&rc_bank_account_repository),
            Rc::clone(&rc_transaction_repository),
        );
        let open_bank_account = OpenBankAccount::new(Rc::clone(&rc_bank_account_repository));

        Self {
            open_bank_account,
            bank_account_repository: rc_bank_account_repository,
            initiate_transaction,
            transaction_repository: rc_transaction_repository,
        }
    }
}
