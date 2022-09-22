use crate::application::ports::{
    bank_account_dto::BankAccountDTO,
    bank_account_repository::BankAccountRepository,
};

use super::super::domain::bank_account::BankAccount;

pub fn open_bank_account(bank_repository: Box<dyn BankAccountRepository>) -> BankAccount {
    let account = BankAccount::new(0);
    let bank_account_dto = BankAccountDTO::new(
        account.get_id(),
        account.get_account_balance(),
        account.get_overdraft_amount(),
    );
    let _ = bank_repository.insert(bank_account_dto);
    // TODO: error handling here
    // before return result
    account
}
