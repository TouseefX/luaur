use crate::functions::is_prim::is_prim;
use crate::records::primitive_type::PrimitiveType;
use crate::type_aliases::type_id::TypeId;

pub fn is_thread(ty: TypeId) -> bool {
    is_prim(ty, PrimitiveType::Thread)
}
