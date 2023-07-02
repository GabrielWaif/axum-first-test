use crate::{persistence::products_repository::ProductsRepository, 
    api::dtos::product_dtos::CreateProductDto,
    domain::entities::product_entity::ProductEntity};

pub struct ProductsService {
    products_repository: ProductsRepository
}

impl ProductsService {
    pub fn new() -> ProductsService {
        ProductsService {
            products_repository: ProductsRepository::new(),
        }
    }

    pub fn add_product(&self, createProductDto: CreateProductDto) -> Result<ProductEntity, String> {
        return self.products_repository.add_product(createProductDto);
    }

    pub fn find_products(&self) -> Result<Vec<ProductEntity>, String> {
        return self.products_repository.find_products();
    }
}
