use domain::models::users::repository::IUserRepository;
use serde::Deserialize;

use crate::common::error::CommandError;

#[derive(Deserialize)]
pub struct DeleteUserByIdCommand {
    pub id: i32
}

pub async fn handle<UserRepository: IUserRepository>(request: DeleteUserByIdCommand) -> Result<(), CommandError> {
    UserRepository::delete(request.id).await?;
    Ok(())
}