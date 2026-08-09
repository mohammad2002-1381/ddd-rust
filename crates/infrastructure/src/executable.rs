use sea_orm::{DatabaseConnection, DbErr, DeleteMany, DeleteResult, EntityTrait};

pub trait IDeletable<E>
where E: EntityTrait {
    fn exec_async(self, db_connection: &DatabaseConnection) -> impl Future<Output = Result<DeleteResult, DbErr>> + Send;
}

impl<E> IDeletable<E> for DeleteMany<E>
where E: EntityTrait {
    async fn exec_async(self, db_connection: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
        self.exec(db_connection).await
    }
}