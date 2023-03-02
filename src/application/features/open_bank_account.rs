use crate::application::ports::bank_account_repository::BankAccountRepository;
use uuid::Uuid;

use super::{super::domain::bank_account::BankAccount, feature::Feature};

pub struct OpenBankAccount {
    bank_account_repository: Box<dyn BankAccountRepository>,
}

impl OpenBankAccount {
    pub fn new(bank_account_repository: Box<dyn BankAccountRepository>) -> Self {
        Self {
            bank_account_repository,
        }
    }
}

impl Feature<String, BankAccount> for OpenBankAccount {
    fn execute(&self, option_fullname: Option<String>) -> Result<BankAccount, String> {
        let id = Uuid::new_v4().to_string();
        let bank_account = BankAccount::new(
            id, 
            option_fullname.unwrap(),
            0
        );
        match self.bank_account_repository.insert(&bank_account) {
            Ok(()) => Ok(bank_account),
            Err(err) => Err(err),
        }
    }
}
