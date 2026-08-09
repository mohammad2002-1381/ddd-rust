use crate::mapper::IMapper;

pub trait ProjectToVecExt {
    type Item;
    
    fn project_to<T>(self) -> Vec<T>
    where
        Self::Item: IMapper<T>;
}

impl<E> ProjectToVecExt for Vec<E> {
    type Item = E;

    fn project_to<T>(self) -> Vec<T>
    where
        E: IMapper<T>,
    {
        self.into_iter().map(|item| item.map()).collect()
    }
}