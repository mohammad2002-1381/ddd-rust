use domain::models::{base_entity::IBaseEntity, users::token::Token};
use sea_orm::{ActiveValue::{NotSet, Set}, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub token: String,
    pub refresh_token: String,
    pub user_id: i32,
    pub expires: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl From<&Token> for ActiveModel {
    fn from(p: &Token) -> Self {
        let id = p.id();
        ActiveModel {
            id: if let Some (id) = id { Set(id) } else { NotSet },
            created_at: Set(p.base().created_at),
            updated_at: Set(p.base().updated_at),
            token: Set(p.token().to_string()),
            refresh_token: Set(p.refresh_token().to_string()),
            user_id: Set(p.user_id()),
            expires: Set(p.expires()),
        }
    }
}

impl From<Model> for Token {
    fn from(value: Model) -> Token {
        Token::new_model(value.id, value.token, value.refresh_token, value.user_id, value.expires, value.created_at, value.updated_at)
    }
}