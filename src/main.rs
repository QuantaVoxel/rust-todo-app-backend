use dotenv::dotenv;

pub mod app;
pub mod common;
pub mod config;
pub mod core;
pub mod entity;
pub mod error;
pub mod modules;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    app::start().await;
}
