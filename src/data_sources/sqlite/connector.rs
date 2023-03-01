use rusqlite::{Connection, Params};

use super::consts::SQL_DATABASE;

pub struct SqliteConnector {
    conn: Connection,
}

impl SqliteConnector {
    pub fn new() -> Self {
        let connection_result = Connection::open_in_memory();

        let conn = match connection_result {
            Ok(conn) => conn,
            Err(_) => panic!("Failed connecting to sqlite"),
        };

        conn.execute(
            SQL_DATABASE,
            (), // empty list of parameters.
        );

        Self { conn }
    }

    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize, rusqlite::Error> {
        self.conn.execute(sql, params)
    }
}
