pub mod errors;
pub trait IBaseQueryService<DTO, Id> {
    fn find_by_id(id: Id) -> impl Future<Output = Result<Option<DTO>, errors::QueryError>> + Send;
}