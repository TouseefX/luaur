use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub type UpperBounds = DenseHashMap<TypeId, Vec<(Location, TypeId)>>;
