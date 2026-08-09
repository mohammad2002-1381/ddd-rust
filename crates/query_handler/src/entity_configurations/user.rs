use read_model::{dto::user_dto::UserDto, mapper::IMapper, models::user::{User, UserRoleType}};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
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

impl From<Model> for User {
    fn from(value: Model) -> User {
        User::new(
            &value.first_name,
            &value.last_name,
            &value.email,
            &value.password_hash,
            value.role,
            value.is_active
        )
    }
}

impl IMapper<UserDto> for Model {
    fn map(self) -> UserDto {
        UserDto {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            password_hash: self.password_hash,
            role: self.role,
            is_active: self.is_active,
            created_at: self.created_at.to_rfc3339(),
            updated_at: self.updated_at.to_rfc3339()
        }
    }
}