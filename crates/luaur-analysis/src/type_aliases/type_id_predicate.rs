use crate::type_aliases::type_id::TypeId;

pub type TypeIdPredicate = alloc::boxed::Box<dyn Fn(TypeId) -> Option<TypeId>>;
