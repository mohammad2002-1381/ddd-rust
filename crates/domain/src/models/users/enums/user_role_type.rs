use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize, Serialize
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Text"
)]
pub enum UserRoleType {
    #[sea_orm(string_value = "admin")]
    Admin,

    #[sea_orm(string_value = "user")]
    User
}