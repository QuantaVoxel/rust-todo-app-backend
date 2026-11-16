use dotenv::dotenv;

pub mod modules;
pub mod app;
pub mod config;
pub mod error;
pub mod common;
pub mod utils;
pub mod core;

#[tokio::main]
async fn main() {

    dotenv().ok();
    tracing_subscriber::fmt().init();

    
    app::start().await;

}
