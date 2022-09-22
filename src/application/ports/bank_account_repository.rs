use super::bank_account_dto::BankAccountDTO;

pub trait BankAccountRepository {
    fn insert(&self, bank_account_dto: BankAccountDTO) -> Result<(), String>;
}
