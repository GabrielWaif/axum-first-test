mod api;
mod server;
mod domain;
mod persistence;
mod services;

use server::server_manager::ServerManager;

#[tokio::main]
async fn main() {
    let server_manager = ServerManager::new();
    server_manager.start().await;
}
