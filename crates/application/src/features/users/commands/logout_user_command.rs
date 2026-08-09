use crate::common::error::CommandError;

pub struct LogoutUserCommand {
    pub token: String
}

impl LogoutUserCommand {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

pub async fn handle(_: LogoutUserCommand) -> Result<(), CommandError> {
    // self.token_repository.delele_many()
    //     .filter(token::Column::Token.eq(request.token))
    //     .exec_async()
    //     .await?;

    Ok(())
}