use std::sync::Mutex;

use crate::domain::entities::product_entity::ProductEntity;
use crate::api::dtos::product_dtos::CreateProductDto;

pub struct ProductsRepository {
    current_id: Mutex<i32>,
    products: Mutex<Vec<ProductEntity>>
}

impl ProductsRepository {
    pub fn new() -> ProductsRepository {
        return ProductsRepository {
            products: Mutex::new(vec![]),
            current_id: Mutex::new(1)
        }
    }

    fn increment_id(&self) -> i32 {
        let mut current_id = self.current_id.lock().unwrap();
        let old_id = current_id.clone();
        *current_id += 1;
        return old_id;
    }

    pub fn add_product(&self, createProductDto: CreateProductDto) -> Result<ProductEntity, String> {
        let product = ProductEntity {
            id: self.increment_id(),
            name: createProductDto.name,
            description: createProductDto.description,
            price: createProductDto.price,
            brand: createProductDto.brand,
        }; 

        let mut products = self.products.lock().unwrap();
        products.push(product.clone());

        return Ok(product);
    }

    pub fn find_products(&self) -> Result<Vec<ProductEntity>, String> {
        let mut products = self.products.lock().unwrap();
        let cloned_products: Vec<ProductEntity> = products.iter().cloned().collect();
        return Ok(cloned_products);
    }
}
