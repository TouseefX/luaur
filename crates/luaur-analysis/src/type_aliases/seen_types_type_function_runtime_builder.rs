use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub type SeenTypes = DenseHashMap<TypeId, TypeFunctionTypeId>;
