use domain::models::products::repository::IProductRepository;
use serde::Deserialize;

use crate::{common::error::CommandError};

#[derive(Deserialize)]
pub struct DeleteProductCommand {
    pub id: i32,
}

pub async fn handle<PR: IProductRepository>(command: DeleteProductCommand, user_id: i32) -> Result<(), CommandError> {
    let product = PR::find_by_id_user_id(command.id, user_id).await?;
    if let Some(_) = product {
        match PR::delete(command.id).await {
            Ok(rows) => {
                Ok(rows)
            },
            Err(e) => Err(CommandError::Internal(e.to_string())),
        }
    }
    else {
        return Err(CommandError::NotFound("product not found".to_string()));
    }
    
}