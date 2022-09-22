use crate::application::ports::{
    bank_account_dto::{self, BankAccountDTO},
    bank_account_repository::BankAccountRepository,
};

use super::super::domain::bank_account::BankAccount;

pub fn open_bank_account(bankRepository: Box<dyn BankAccountRepository>) -> BankAccount {
    let account = BankAccount::new(0);
    let bank_account_dto = BankAccountDTO::new(
        account.get_id(),
        account.get_account_balance(),
        account.get_overdraft_amount(),
    );
    bankRepository.insert(bank_account_dto);
    account
}
