pub const SQL_DATABASE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS transactions (
        id                  TEXT PRIMARY KEY,
        _from                TEXT NOT NULL,
        _to                  TEXT NOT NULL,
        amount              INT NOT NULL,
        created_at          DATETIME DEFAULT CURRENT_TIMESTAMP
    );
    CREATE TABLE IF NOT EXISTS bankaccounts (
        id                  TEXT PRIMARY KEY,
        fullname            TEXT NOT NULL,
        account_balance     INT NOT NULL,
        created_at          DATETIME DEFAULT CURRENT_TIMESTAMP
    );
    INSERT OR IGNORE INTO bankaccounts (id, fullname, account_balance) VALUES ('0', 'Shine', 100000000);
"#;
