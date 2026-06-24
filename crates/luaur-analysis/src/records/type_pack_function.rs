use crate::type_aliases::reducer_function::ReducerFunction;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct TypePackFunction {
    pub name: alloc::string::String,
    pub reducer: ReducerFunction,
    pub can_reduce_generics: bool,
}
