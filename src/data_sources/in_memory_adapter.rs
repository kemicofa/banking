use crate::application::{
    domain::bank_account::BankAccount, ports::bank_account_repository::BankAccountRepository,
};
use std::collections::HashMap;

pub struct InMemoryAdapter {
    cache: HashMap<String, String>,
}

impl InMemoryAdapter {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

impl BankAccountRepository for InMemoryAdapter {
    fn insert(self: &InMemoryAdapter, bank_account_dto: &BankAccount) -> Result<(), String> {
        let result = serde_json::to_string(&bank_account_dto);

        match result {
            Ok(json) => self
                .cache
                .to_owned()
                .insert(bank_account_dto.get_id(), json),
            Err(_) => return Err("Failed serializing data".to_string()),
        };

        Ok(())
    }
}
