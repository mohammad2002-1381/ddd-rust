use crate::models::base_entity::IBaseEntity;

pub trait IAggregateRoot<TId> : IBaseEntity<TId> {}