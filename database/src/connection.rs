use rusqlite::{Connection, Result};

pub fn execute<T>(execute_fn: impl Fn(&Connection) -> Result<T>) -> Result<T> {
    let conn = Connection::open("./zzhack.db")?;
    let result = execute_fn(&conn)?;

    conn.close().map_err(|(_, err)| err)?;

    Ok(result)
}
