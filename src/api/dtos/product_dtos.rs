use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateProductDto{
    pub name: String,
    pub description: String,
    pub price: f32,
    pub brand: String
}
