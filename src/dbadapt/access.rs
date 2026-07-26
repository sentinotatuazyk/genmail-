use odbc_api::{Connection, ConnectionOptions, ResultSetMetadata, Cursor};
use odbc_api::buffers::TextRowSet;
use std::path::Path;
use super::{DBConnection, DBError, odbc_env};

pub struct AccessConnection {
    conn: Connection<'static>,
}

impl AccessConnection {
    pub fn connect(db_path: &Path) -> Result<Self, DBError> {
        let connection_string = format!(
            "Driver={{Microsoft Access Driver (*.mdb, *.accdb)}}; Dbq={};",
            db_path.display()
        );

        let conn = odbc_env().connect_with_connection_string(
            &connection_string,
            ConnectionOptions::default(),
        )?;
        Ok(Self { conn })
    }
}

impl DBConnection for AccessConnection {
    fn execute(&self, sql: &str) -> Result<(), DBError> {
        self.conn.execute(sql, (), None)?;
        Ok(())
    }

    fn query_rows(&self, sql: &str) -> Result<(Vec<String>, Vec<Vec<String>>), DBError> {
        let mut cursor = self.conn.execute(sql, (), None)?
            .ok_or(DBError::ConnectToNone)?;

        // Zamiast ręcznego num_result_cols + col_name, użyj column_names() — prostsze i pewne API
        let headers: Vec<String> = cursor
            .column_names()?
            .collect::<Result<Vec<String>, _>>()?;

        let num_cols = headers.len();

        let mut buffers = TextRowSet::for_cursor(100, &mut cursor, Some(4096))?;
        let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

        let mut rows: Vec<Vec<String>> = Vec::new();

        while let Some(batch) = row_set_cursor.fetch()? {
            for row_index in 0..batch.num_rows() {
                let mut row = Vec::with_capacity(num_cols);
                for col_index in 0..num_cols {
                    let value = batch
                        .at_as_str(col_index, row_index)
                        .unwrap_or(None)
                        .unwrap_or("")
                        .to_string();
                    row.push(value);
                }
                rows.push(row);
            }
        }
        Ok((headers, rows))
    }


    fn list_tables(&self) -> Result<Vec<String>, DBError> {
        let mut tables = Vec::new();

        for row in self.conn.tables("","","","TABLES")? {
            let row: odbc_api::TablesRow = row?;
            if let Ok(Some(table_name)) = row.table.as_str() {
                tables.push(table_name.to_string());
            }
        }

        Ok(tables)
    }
}
