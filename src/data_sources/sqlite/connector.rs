use std::{fs::{self, File}, path::Path};

use rusqlite::{Connection, Params, OpenFlags, Statement, Error};

use super::consts::SQL_DATABASE;

pub struct SqliteConnector {
    conn: Connection,
}

const DATABASE_DIR: &'static str = "/var/tmp/kemicofa.banking";
const DATABASE_NAME: &'static str = "data.db";

impl SqliteConnector {
    pub fn new() -> Self {
        let path = DATABASE_DIR.to_string() + "/" + DATABASE_NAME;
        fs::create_dir_all(DATABASE_DIR)
            .expect("Unable to create directories for sqlite database");
        if !Path::new(&path.to_string()).exists() {
            File::create(path.clone()).expect("Could not create database file for sqlite");
        }
        let connection_result = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE);

        let conn = match connection_result {
            Ok(conn) => conn,
            Err(err) => panic!("{}", err),
        };

        conn.execute(
            SQL_DATABASE,
            (), // empty list of parameters.
        ).unwrap();

        Self { conn }
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize, rusqlite::Error> {
        self.conn.execute(sql, params)
    }

    pub fn prepare(&self, sql: &str) -> Result<Statement, Error> {
        self.conn.prepare(sql)
    }
}
