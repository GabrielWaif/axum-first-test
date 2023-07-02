mod product_routes;

use crate::{api::routes::product_routes::ProductRoutes, ApiDoc};

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
        router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));
        RouteManager { router }
    }

}
