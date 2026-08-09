use read_model::queryable::pagination::{PaginatedResult, PaginationParams};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QuerySelect, Select};

pub trait IPaginationExtension<E>
where E: EntityTrait,
<E as EntityTrait>::Model: Sync {
    fn paged_list(self, db: &DatabaseConnection, pagination: &PaginationParams) -> impl Future<Output = Result<PaginatedResult<<E as EntityTrait>::Model>, DbErr>>;
}

impl<E> IPaginationExtension<E> for Select<E>
where E: EntityTrait,
<E as EntityTrait>::Model: Sync {
    async fn paged_list(self, db: &DatabaseConnection, pagination: &PaginationParams) -> Result<PaginatedResult<<E as EntityTrait>::Model>, DbErr> {
        let total_count = self.clone().count(db).await?;
        let items = self.offset((pagination.page_number - 1) * pagination.page_size).limit(pagination.page_size).all(db).await?;
        Ok(
            PaginatedResult{
                items,
                total_count,
                page_number: pagination.page_number,
                page_size: pagination.page_size,
            }
        )
    }
}
