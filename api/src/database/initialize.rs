use sea_orm::{DatabaseConnection, DbBackend};

pub fn initialize(conn: &DatabaseConnection) {
    let db_sqlite = DbBackend::Sqlite;
}
