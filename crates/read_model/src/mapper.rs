pub trait IMapper<T> {
    fn map(self) -> T;
}