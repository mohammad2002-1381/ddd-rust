pub struct GetProductByIdQuery {
    pub id: i32,
}

impl GetProductByIdQuery {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}