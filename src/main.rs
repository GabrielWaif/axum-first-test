mod api;
mod server;
mod domain;

use server::server_manager::ServerManager;

#[tokio::main]
async fn main() {
    let server_manager = ServerManager::new();
    server_manager.start().await;
}
