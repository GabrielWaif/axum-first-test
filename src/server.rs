pub mod server_manager {
    use std::net::SocketAddr;
    use crate::api::routes::RouteManager;
    use axum::Server;

    pub struct ServerManager {}

    impl ServerManager {
        pub fn new() -> ServerManager {
            return ServerManager{};
        }
        pub async fn start(&self) {
            let route_manager = RouteManager::new();

            let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

            Server::bind(&addr)
                .serve(route_manager.router.into_make_service())
                .await
                .unwrap();
        }
    }
}
