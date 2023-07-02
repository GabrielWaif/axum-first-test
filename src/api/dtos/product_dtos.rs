use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateProductDto{
    pub name: String,
    pub description: String,
    pub price: f32,
    pub brand: String
}
