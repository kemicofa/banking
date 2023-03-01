use std::{collections::HashMap};
use crate::application::{ports::{
    bank_account_repository::BankAccountRepository,
}, dtos::bank_account_dto::BankAccountDTO, errors::Error};

pub struct InMemoryAdapter {
    cache: HashMap<String, String>,
}

impl InMemoryAdapter {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }
}

impl BankAccountRepository for InMemoryAdapter {
    fn insert(self: &InMemoryAdapter, bank_account_dto: BankAccountDTO) -> Result<(), Error> {
        let result = serde_json::to_string(&bank_account_dto);

        match result {
            Ok(json) => self.cache.to_owned().insert(bank_account_dto.get_id(), json),
            Err(_) => return Err(Error::new("Failed serializing data")),
        };

        Ok(())
    }
}
