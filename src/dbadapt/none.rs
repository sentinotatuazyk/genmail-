use super::{DBConnection, DBError};

pub struct NullConnection;

impl DBConnection for NullConnection {
    fn execute(&self, _sql: &str) -> Result<(), DBError> {
        Err(DBError::NotConfigured)
    }

    fn query_rows(&self, _sql: &str) -> Result<Vec<Vec<String>>, DBError> {
        Err(DBError::NotConfigured)
    }

    fn is_configured(&self) -> bool {
        false
    }
}
