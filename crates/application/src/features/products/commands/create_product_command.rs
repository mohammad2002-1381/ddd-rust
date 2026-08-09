use domain::models::products::{product::Product, repository::IProductRepository};
use serde::Deserialize;

use crate::{common::error::CommandError};

#[derive(Deserialize)]
pub struct CreateProductCommand {
    pub name: String,
    pub price: i64,
}

pub async fn handle<PR: IProductRepository>(command: CreateProductCommand, user_id: i32) -> Result<i32, CommandError> {
    let product = Product::new(&command.name, command.price, true, user_id);
    let pr = PR::create(&product).await?;
    Ok(pr)
}