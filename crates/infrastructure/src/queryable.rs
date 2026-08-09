use sea_orm::{DatabaseConnection, EntityTrait, Select, SelectTwo};

pub trait IQueryable<E>
where E: EntityTrait {
    fn first_async(self, db_connection: &DatabaseConnection) -> impl std::future::Future<Output = Result<Option<<E as EntityTrait>::Model>, sea_orm::DbErr>> + Send;
}

pub trait IQueryableTwo<E, F>
where
    E: EntityTrait,
    F: EntityTrait {
    fn first_async(self, db_connection: &DatabaseConnection) -> impl std::future::Future<Output = Result<Option<(<E as EntityTrait>::Model, Option<<F as EntityTrait>::Model>)>, sea_orm::DbErr>> + Send;
}

impl<E> IQueryable<E> for Select<E>
where E: EntityTrait {
    async fn first_async(self, db_connection: &DatabaseConnection) -> Result<Option<<E as EntityTrait>::Model>, sea_orm::DbErr> {
        self.one(db_connection).await
    }
}

impl<E, F> IQueryableTwo<E, F> for SelectTwo<E, F>
where
    E: EntityTrait,
    F: EntityTrait {
    async fn first_async(self, db_connection: &DatabaseConnection) -> Result<Option<(<E as EntityTrait>::Model, Option<<F as EntityTrait>::Model>)>, sea_orm::DbErr> {
        self.one(db_connection).await
    }
}