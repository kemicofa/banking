use crate::application::{dtos::bank_account_dto::BankAccountDTO, errors::Error};

pub trait BankAccountRepository {
    fn insert(&self, bank_account_dto: BankAccountDTO) -> Result<(), Error>;
}
