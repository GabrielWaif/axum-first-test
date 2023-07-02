use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::domain::entities::product_entity::ProductEntity;

#[derive(Deserialize, Serialize, ToSchema)]
#[aliases(
    ResultProductEntity = ResultEntity<ProductEntity>,
    ResultProductEntities = ResultEntity<Vec<ProductEntity>>,
)]
pub struct ResultEntity<T> {
    pub status: i32,
    pub result: Option<T>,
    pub success: bool,
    pub notifications: Vec<String>,
    pub errors: Vec<String>
}

impl <T>ResultEntity<T> {
    pub fn new(status: i32, result: Option<T>, success: bool, notifications: Vec<String>, errors: Vec<String>) -> ResultEntity<T> {
        ResultEntity {
            status,
            result,
            success,
            notifications,
            errors,
        }
    }
}
