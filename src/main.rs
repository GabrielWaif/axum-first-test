mod api;
mod server;
mod domain;
mod persistence;
mod services;

use api::{dtos::product_dtos::CreateProductDto, utils::result_entity::{ResultEntity, ResultProductEntity}};
use domain::entities::product_entity::ProductEntity;
use server::server_manager::ServerManager;
use utoipa::OpenApi;

use api::controllers::products_controllers::{
    __path_find_products,
    __path_create_product
};

#[derive(OpenApi)]
#[openapi(
    paths (
        find_products,
        create_product
    ),
    components (
        schemas(
            CreateProductDto,
            ProductEntity,
            ResultProductEntity
       )
   ),
   tags (
       (name = "Bla")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let server_manager = ServerManager::new();
    server_manager.start().await;
}
