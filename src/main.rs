
use todo_app::{infrastructure,core};
use migration::{Migrator, MigratorTrait};

use dotenv::dotenv;
use salvo::prelude::*;
use std::env;
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    tracing::info!("Starting Todo App Server ...");

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);
    let socket_addr: SocketAddr = addr
        .parse()
        .expect("Format port tidak valid");

    let db = infrastructure::db::init_pool().await.expect("Failed to Connect Database");

    Migrator::up(&db, None)
        .await
        .expect("Proses migrasi database gagal");

    tracing::info!("Migrasi database selesai diterapkan.");

    let acceptor = TcpListener::new(socket_addr).bind().await;
    let router : Router = Router::new()
        .hoop(affix_state::inject(db))
        .push(core::router::routers());

    println!("{router:?}");

    tracing::info!("Starting Todo App Server on {}", socket_addr);

    Server::new(acceptor).serve(router).await;
}
