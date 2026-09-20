use domain::models::products::repository::IProductRepository;
use serde::Deserialize;

use crate::{common::error::CommandError};

#[derive(Deserialize)]
pub struct UpdateProductCommand {
    pub id: i32,
    pub name: String,
    pub price: i64,
    pub is_active: bool
}

pub async fn handle<PR: IProductRepository>(command: UpdateProductCommand, user_id: i32) -> Result<i32, CommandError> {
    let product = PR::find_by_id_user_id(command.id, user_id)
        .await?;
    match product {
        Some(mut pr) => {
            pr.update(command.name, command.price, command.is_active);
            PR::update(&mut pr).await?;
            return Ok(command.id);
        },
        None => {
            return Err(CommandError::NotFound(String::from("product not found.")));
        },
    }
}