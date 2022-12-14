use std::{collections::HashMap, str::FromStr, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::application::ports::{
    bank_account_dto::BankAccountDTO, bank_account_repository::BankAccountRepository,
};

pub struct InMemoryAdapter {
    cache: HashMap<String, String>,
}

impl InMemoryAdapter {
    pub fn new() -> InMemoryAdapter {
        InMemoryAdapter { cache: HashMap::new() }
    }
}

impl BankAccountRepository for InMemoryAdapter {
    fn insert(self: &InMemoryAdapter, bank_account_dto: BankAccountDTO) -> Result<(), String> {
        let result = serde_json::to_string(&bank_account_dto);

        match result {
            Ok(json) => self.cache.to_owned().insert(bank_account_dto.get_id(), json),
            Err(_) => return Err(String::from("Failed serializing data")),
        };

        Ok(())
    }
}
