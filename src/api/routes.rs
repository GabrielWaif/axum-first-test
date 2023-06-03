mod product_routes;

use crate::api::routes::product_routes::ProductRoutes;

use axum::Router;

trait RouteMapper {
    fn map_routes(&self, router: Router) -> Router;
}

pub struct RouteManager {
    pub router: Router,
}

fn add_routes<T: RouteMapper>(router: Router, route_mapper: T) -> Router {
    let aux_router: Router;
    aux_router = route_mapper.map_routes(router);
    return aux_router;
}

impl RouteManager {
    pub fn new() -> RouteManager {
        let mut router = Router::new();
        router = add_routes(router, ProductRoutes{});
        RouteManager { router }
    }

}
