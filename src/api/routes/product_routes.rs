use axum::routing::get;
use crate::api::routes::RouteMapper;
use crate::api::controllers::products_controllers::create_product;

use axum::Router;

pub struct ProductRoutes;

impl RouteMapper for ProductRoutes {
    fn map_routes(&self, router: Router) -> Router {
        let router = router.route("/test", get(create_product));
        return router;
    }
}
