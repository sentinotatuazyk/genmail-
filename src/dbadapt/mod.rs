mod access;
mod sqlite;
mod none;

pub use access::AccessConnection;
pub use sqlite::SqliteConnection;
pub use none::NullConnection;

use crate::app::configshandler::{Config, DBTypes};

use odbc_api::Environment;
use std::sync::OnceLock;

static ENV: OnceLock<Environment> = OnceLock::new();

pub(super) fn odbc_env() -> &'static Environment {
    ENV.get_or_init(|| Environment::new().expect("Failed to crate ODBC environment"))
}

pub trait DBConnection {
    fn execute(&self, sql: &str) -> Result<(), DBError>;
    fn query_rows(&self, sql: &str) -> Result<(Vec<String>,Vec<Vec<String>>), DBError>;
    fn list_tables(&self) -> Result<Vec<String>, DBError>;
    fn is_configured(&self) ->bool {false}
}

pub fn connect(config: &Config) -> Result<Box<dyn DBConnection>, DBError> {
    let db_path = config.db_path.as_ref().ok_or(DBError::NotConfigured)?;
    match config.db_type {
        DBTypes::Access => Ok(Box::new(AccessConnection::connect(db_path)?)),
        DBTypes::SQLite => Ok(Box::new(SqliteConnection::connect(db_path)?)),
        DBTypes::None => 
            Err(DBError::ConnectToNone),
    }
}

#[derive(Debug)]
pub enum DBError{
    NotConfigured,
    ConnectToNone,
    Odbc(odbc_api::Error),
}

impl From<odbc_api::Error> for DBError {
    fn from(e: odbc_api::Error) -> Self {
        DBError::Odbc(e)
    }
}

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            DBError::NotConfigured =>write!(f, "Baza danych nie zostala jeszcze skonfigurowana!"),
            DBError::ConnectToNone => write!(f, "Spróbowano połączyć się z bazą danych typu None!"),
            DBError::Odbc(e) => write!(f, "Blad ODBC: {e}"),
        }
    }
}

impl std::error::Error for DBError {}

pub fn empty() -> Box<dyn DBConnection> {
    Box::new(NullConnection)
}
