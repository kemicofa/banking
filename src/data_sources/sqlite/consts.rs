pub const SQL_DATABASE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS bankaccounts (
        id                  TEXT PRIMARY KEY,
        account_balance     INT NOT NULL,
        created_at          DATETIME DEFAULT CURRENT_TIMESTAMP
    )
"#;
