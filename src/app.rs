use std::net::SocketAddr;
use std::time::Duration;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use salvo::server::ServerHandle;
use migration::{Migrator, MigratorTrait};

use crate::config::Settings;
use crate::core::db::connection;
use crate::core::metrics::{init_start_time, set_shutting_down};
use crate::core::middleware::error_catcher;
use crate::core::routes;

pub async fn start() {
    init_start_time();
    let settings = Settings::load();

    tracing::info!("Starting Todo App Server ...");

    let addr = format!("{}:{}", settings.app_host, settings.app_port);
    let socket_addr: SocketAddr = addr.parse().expect("Format APP_HOST / APP_PORT tidak valid");

    let acceptor = TcpListener::new(socket_addr).bind().await;
    let router: Router = routes::routes();
    let service = Service::new(router).catcher(Catcher::default().hoop(error_catcher));

    // Ensure uploads directory exists
    tokio::fs::create_dir_all("uploads").await.ok();

    // Check DB connection & run migrations
    db_starting().await;

    // Optional auto-seeding when SEED_DEMO_DATA=true or --seed argument is provided
    let should_seed = std::env::var("SEED_DEMO_DATA")
        .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
        .unwrap_or(false)
        || std::env::args().any(|a| a == "--seed");

    if should_seed {
        let db = connection::get_db_connection();
        match crate::core::seeder::seed_demo_data(db).await {
            Ok(summary) => {
                tracing::info!(
                    "Demo Seeder: {} (demo_user: {} / {})",
                    summary.message,
                    summary.demo_account.username,
                    summary.demo_account.password
                );
            }
            Err(err) => {
                tracing::warn!("Gagal menjalankan demo seeder otomatis: {:?}", err);
            }
        }
    }

    // Spawn Background Worker with Graceful Shutdown channel
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    tokio::spawn(async move {
        crate::core::worker::start_worker(shutdown_rx).await;
    });

    let server = Server::new(acceptor).max_connections(10_000);
    let handle = server.handle();
    tokio::spawn(shutdown_signal(handle, shutdown_tx));

    tracing::info!("Starting Todo App Server on http://{}", socket_addr);

    server.serve(service).await;
    tracing::info!("Todo App Server telah berhenti secara bersih (Graceful Shutdown selesai).");
}

async fn shutdown_signal(handle: ServerHandle, shutdown_tx: tokio::sync::watch::Sender<bool>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Gagal memasang Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm = signal(SignalKind::terminate()).expect("Gagal memasang SIGTERM handler");
        let mut sigquit = signal(SignalKind::quit()).expect("Gagal memasang SIGQUIT handler");
        tokio::select! {
            _ = sigterm.recv() => {}
            _ = sigquit.recv() => {}
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Sinyal Ctrl+C (SIGINT) diterima, memulai graceful shutdown..."),
        _ = terminate => tracing::info!("Sinyal SIGTERM/SIGQUIT diterima, memulai graceful shutdown..."),
    }

    set_shutting_down(true);
    let _ = shutdown_tx.send(true);

    // Drain in-flight requests up to 25 seconds
    handle.stop_graceful(Duration::from_secs(25));
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
