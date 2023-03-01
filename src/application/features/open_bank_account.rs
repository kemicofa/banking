use crate::application::{ports::{
    bank_account_repository::BankAccountRepository,
}, dtos::bank_account_dto::BankAccountDTO};
use uuid::Uuid;

use super::{super::domain::bank_account::BankAccount, feature::Feature};

pub struct OpenBankAccount<'a> {
    bank_account_repository: &'a dyn BankAccountRepository
}

impl OpenBankAccount<'_> {
    pub fn new(bank_account_repository: &dyn BankAccountRepository) -> Self {
        Self {
            bank_account_repository
        }
    }
}

impl Feature<(), BankAccountDTO> for OpenBankAccount<'_> {
    fn execute(&self, _: Option<()>) -> Result<BankAccountDTO, crate::application::errors::Error> {
        let id =  Uuid::new_v4().to_string();
        let account = BankAccount::new(id, 0);
        let bank_account_dto = BankAccountDTO::new(
            account.get_id(),
            account.get_account_balance(),
            account.get_overdraft_amount(),
        );
        let _ = self.bank_account_repository.insert(bank_account_dto.clone());
        // TODO: error handling here
        // before return result
        Ok(bank_account_dto)
    }
}
