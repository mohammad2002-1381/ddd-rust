use domain::models::{base_entity::IBaseEntity, users::{enums::user_role_type::UserRoleType, user::User}};
use sea_orm::{ActiveValue::{NotSet, Set}, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub role: UserRoleType,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl From<&User> for ActiveModel {
    fn from(p: &User) -> Self {
        let id = p.id();
        ActiveModel {
            id: if let Some (id) = id { Set(id) } else { NotSet },
            first_name: Set(p.first_name().to_owned()),
            last_name: Set(p.last_name().to_owned()),
            email: Set(p.email().to_owned()),
            password_hash: Set(p.password_hash().to_owned()),
            role: Set(p.role()),
            is_active: Set(p.is_active()),
            created_at: Set(p.base().created_at),
            updated_at: Set(p.base().updated_at),
        }
    }
}

impl From<Model> for User {
    fn from(value: Model) -> User {
        User::new_model(value.id, value.first_name, value.last_name, value.email, value.password_hash, value.role, value.is_active, value.created_at, value.updated_at)
    }
}