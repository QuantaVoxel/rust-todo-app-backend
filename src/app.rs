use std::env;
use std::net::SocketAddr;
use crate::core::db::connection;
use migration::{Migrator, MigratorTrait};
use salvo::prelude::*;
use crate::core::routes;

pub async  fn start() {


    tracing::info!("Starting Todo App Server ...");

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);
    let socket_addr: SocketAddr = addr.parse().expect("Format port tidak valid");

    let acceptor = TcpListener::new(socket_addr).bind().await;
    let router: Router = Router::new().push(routes::routes());

    // check db connection
    db_starting().await;

    println!("{router:?}");

    tracing::info!("Starting Todo App Server on {}", socket_addr);


    Server::new(acceptor).serve(router).await;

}

async fn db_starting(){

    connection::connect()
        .await
        .expect("Database gagal koneksi saat startup");

    let db_connection = connection::get_db_connection();

    Migrator::up(db_connection, None)
        .await
        .expect("Proses migrasi database gagal");

    tracing::info!("Migrasi database selesai diterapkan.");
}