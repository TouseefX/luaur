use crate::type_aliases::reducer_function::ReducerFunction;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct TypeFunction {
    pub name: alloc::string::String,
    pub reducer: ReducerFunction,
    pub can_reduce_generics: bool,
}
