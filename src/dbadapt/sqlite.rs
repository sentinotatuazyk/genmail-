use odbc_api::{Connection, ConnectionOptions};
use crate::app::configshandler::Config;
use super::DBConnection;
use super::odbc_env;
use std::path::Path;
use super::DBError;

pub struct SqliteConnection {
    conn: Connection<'static>,
}

impl SqliteConnection {
    pub fn connect(db_path: &Path) -> Result<Self, DBError> {
        let connection_string = format!(
            "Driver={{SQLite3 ODBC Driver}}; Database={};",
            db_path.display()
        );

        let conn = odbc_env().connect_with_connection_string(
            &connection_string,
            ConnectionOptions::default(),
        )?;
        Ok(Self{conn})
    }
}

impl DBConnection for SqliteConnection{
    fn execute(&self, sql: &str) -> Result<(), DBError> {
        self.conn.execute(sql, (), None)?;
        Ok(())
    }

    fn query_rows(&self, sql: &str) -> Result<Vec<Vec<String>>, DBError> {
        todo!()
    }

}
