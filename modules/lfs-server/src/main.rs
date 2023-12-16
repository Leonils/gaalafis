use crate::{config::ServerConfig, injected_services::new_server_config, server::run_server};
use std::env;
use std::sync::Arc;

mod config;
mod injected_services;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = ServerConfig::from_args(args);
    let services = new_server_config(&config);
    run_server(config, Arc::new(services)).await;
}
