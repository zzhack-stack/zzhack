use sea_orm::{Database, DatabaseConnection, DbErr};

pub type DBResult<T, E = DbErr> = Result<T, E>;

pub async fn get_db_connection() -> DatabaseConnection {
    // By default, sea_orm using sqlx::Pool for connection pool.
    Database::connect("sqlite://../zzhack.db?mode=rwc")
        .await
        .expect("Connect to SQLite failed")
}
