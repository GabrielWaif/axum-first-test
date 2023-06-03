use axum::{Json};

use crate::domain::entities::product_entity::ProductEntity;

use crate::api::dtos::product_dtos::{
    CreateProductDto
};

use crate::api::utils::result_entity::ResultEntity;

pub async fn create_product(create_product_dto: Json<CreateProductDto>) -> Json<ResultEntity<ProductEntity>> {
    let entity = ProductEntity{ 
        id: 1,
        name: create_product_dto.name.clone(),
        description: create_product_dto.description.clone(),
        price: create_product_dto.price.clone(),
        brand: create_product_dto.brand.clone()
    };

    let result = ResultEntity::new(200, entity, true, vec![], vec![]);

    return Json(result);
}
