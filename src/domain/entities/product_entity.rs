use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ProductEntity {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub brand: String
}
