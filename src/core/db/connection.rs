use salvo::__private::tracing::log;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::env;
use std::sync::OnceLock;
use std::time::Duration;

pub static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn connect() -> Result<(), DbErr> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:root@localhost:5432/rust_todo_app".to_string());

    let mut opt = ConnectOptions::new(db_url);

    opt.max_connections(50)
       .min_connections(5)
       .connect_timeout(Duration::from_secs(8))
       .acquire_timeout(Duration::from_secs(8))
       .idle_timeout(Duration::from_secs(10))
       .max_lifetime(Duration::from_secs(60 * 30))
       .sqlx_logging(false)
       .sqlx_logging_level(log::LevelFilter::Info);

    let connection = Database::connect(opt).await?;

    let _ = DB.set(connection);

    println!("✅ Berhasil terhubung ke database!");

    Ok(())
}

pub fn get_db_connection() -> &'static DatabaseConnection {
    DB.get().expect("Database belum terhubung! Pastikan connect() dipanggil di main.rs")
}

pub fn try_get_db_connection() -> Option<&'static DatabaseConnection> {
    DB.get()
}
