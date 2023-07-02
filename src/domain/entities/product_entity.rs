use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProductEntity {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub brand: String
}

impl Clone for ProductEntity {
    fn clone(&self) -> Self {
        ProductEntity {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            price: self.price.clone(),
            brand: self.brand.clone(),
        }
    }
}
