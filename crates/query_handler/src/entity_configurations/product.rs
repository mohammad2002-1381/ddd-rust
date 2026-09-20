use read_model::{dto::product_dto::ProductDto, mapper::IMapper, models::product::Product};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub price: i64,
    pub is_active: bool,
    pub user_id: i32,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl IMapper<ProductDto> for Model {
    fn map(self) -> ProductDto {
        ProductDto {
            id: self.id,
            name: self.name,
            price: self.price,
            created_at: self.created_at.to_rfc3339(),
            updated_at: self.updated_at.to_rfc3339()
        }
    }
}