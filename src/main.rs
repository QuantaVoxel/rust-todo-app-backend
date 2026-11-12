use migration::{Migrator, MigratorTrait};
use todo_app::core;

use dotenv::dotenv;
use salvo::prelude::*;
use std::env;
use std::net::SocketAddr;
use todo_app::infrastructure::db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    tracing::info!("Starting Todo App Server ...");

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);
    let socket_addr: SocketAddr = addr.parse().expect("Format port tidak valid");

    db::connect()
        .await
        .expect("Database gagal koneksi saat startup");

    let db_connection = db::get_db();

    Migrator::up(db_connection, None)
        .await
        .expect("Proses migrasi database gagal");

    tracing::info!("Migrasi database selesai diterapkan.");

    let acceptor = TcpListener::new(socket_addr).bind().await;
    let router: Router = Router::new().push(core::router::routers());

    println!("{router:?}");

    tracing::info!("Starting Todo App Server on {}", socket_addr);

    Server::new(acceptor).serve(router).await;
}
