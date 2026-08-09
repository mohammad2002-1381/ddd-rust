use domain::models::{base_entity::IBaseEntity, products::product::Product};
use sea_orm::{ActiveValue::{NotSet, Set}, entity::prelude::*};

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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<&Product> for ActiveModel {
    fn from(p: &Product) -> Self {
        let id = p.id();
        ActiveModel {
            id: if let Some (id) = id { Set(id) } else { NotSet },
            name: Set(p.name().to_owned()),
            price: Set(p.price()),
            is_active: Set(p.is_active()),
            user_id: Set(p.user_id()),
            created_at: Set(p.base().created_at),
            updated_at: Set(p.base().updated_at),
        }
    }
}

impl From<Model> for Product {
    fn from(model: Model) -> Self {
        Product::new_model(model.id,
        model.name,
        model.price,
        model.is_active,
        model.user_id,model.created_at,model.updated_at)
    }
}