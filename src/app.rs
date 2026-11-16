use std::net::SocketAddr;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use migration::{Migrator, MigratorTrait};

use crate::config::Settings;
use crate::core::db::connection;
use crate::core::middleware::not_found_handler;
use crate::core::routes;

pub async fn start() {
    let settings = Settings::load();

    tracing::info!("Starting Todo App Server ...");

    let addr = format!("{}:{}", settings.app_host, settings.app_port);
    let socket_addr: SocketAddr = addr.parse().expect("Format APP_HOST / APP_PORT tidak valid");

    let acceptor = TcpListener::new(socket_addr).bind().await;
    let router: Router = routes::routes();
    let service = Service::new(router).catcher(Catcher::default().hoop(not_found_handler));

    // Check DB connection & run migrations
    db_starting().await;

    tracing::info!("Starting Todo App Server on http://{}", socket_addr);

    Server::new(acceptor).serve(service).await;
}

async fn db_starting() {
    connection::connect()
        .await
        .expect("Database gagal koneksi saat startup");

    let db_connection = connection::get_db_connection();

    Migrator::up(db_connection, None)
        .await
        .expect("Proses migrasi database gagal");

    tracing::info!("Migrasi database selesai diterapkan.");
}
