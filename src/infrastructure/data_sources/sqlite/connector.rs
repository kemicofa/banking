use std::{
    env,
    fs::{self, File},
    path::Path,
};

use rusqlite::{Connection, Error, OpenFlags, Params, Statement};

use super::consts::SQL_DATABASE;

pub struct SqliteConnector {
    conn: Connection,
}

impl SqliteConnector {
    pub fn new() -> Self {
        let path = env::temp_dir().join("kemicofa.banking").join("data.db");

        if !Path::new(&path).exists() {
            fs::create_dir_all(path.parent().unwrap())
                .expect("Unable to create directories for sqlite database");
            File::create(path.clone()).expect("Could not create database file for sqlite");
        }
        let connection_result =
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE);

        let conn = match connection_result {
            Ok(conn) => conn,
            Err(err) => panic!("{}", err),
        };

        conn.execute_batch(SQL_DATABASE).unwrap();

        Self { conn }
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize, rusqlite::Error> {
        self.conn.execute(sql, params)
    }

    pub fn prepare(&self, sql: &str) -> Result<Statement, Error> {
        self.conn.prepare(sql)
    }
}
