use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub price: i64,
    pub created_at: String,
    pub updated_at: String
}
