use axum::Json;

use crate::domain::entities::product_entity::ProductEntity;

use crate::api::dtos::product_dtos::CreateProductDto;

use axum::extract::Extension;

use crate::api::utils::result_entity::ResultEntity;
use crate::services::product_service::ProductsService;

#[utoipa::path(
    post,
    path = "/products",
    responses (
        (status = 201, body = ResultProductEntity)
    ),
    request_body = CreateProductDto
)]
pub async fn create_product(product_service: Extension<std::sync::Arc<ProductsService>>, create_product_dto: Json<CreateProductDto>) -> Json<ResultEntity<ProductEntity>> {
    let entity = product_service.0.add_product(create_product_dto.0);

    let result: ResultEntity<ProductEntity>;

    match entity {
        Ok(val) => {
            result = ResultEntity::new(200, Some(val), true, vec![], vec![]); 
        }
        Err(err) => {
            result = ResultEntity::new(500, None, true, vec![], vec![]); 
        }
    }

    return Json(result);
}

#[utoipa::path(
    get,
    path = "/products",
    responses (
        (status = 200, body = ResultProductEntities)
    )
)]
pub async fn find_products(product_service: Extension<std::sync::Arc<ProductsService>>) -> Json<ResultEntity<Vec<ProductEntity>>> {
    let entity = product_service.0.find_products();

    let result: ResultEntity<Vec<ProductEntity>>;

    match entity {
        Ok(val) => {
            result = ResultEntity::new(200, Some(val), true, vec![], vec![]); 
        }
        Err(err) => {
            result = ResultEntity::new(500, None, true, vec![], vec![]); 
        }
    }

    return Json(result);
}
