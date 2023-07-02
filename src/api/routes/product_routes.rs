use axum::routing::{get, post};
use crate::api::routes::RouteMapper;
use crate::api::controllers::products_controllers::{create_product, find_products};
use crate::services::product_service::ProductsService;

use axum::Router;

pub struct ProductRoutes {
}

impl RouteMapper for ProductRoutes {
    fn map_routes(&self, router: Router) -> Router {
        let products_service = std::sync::Arc::new(ProductsService::new());

        let router = router
            .route("/products", post(create_product))
            .route("/products", get(find_products))
            .layer(axum::Extension(products_service));
        return router;
    }
}
