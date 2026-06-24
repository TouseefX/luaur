//! @interface-stub
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use alloc::vec::Vec;

impl ConstraintGenerator {
    pub fn add_type_pack(&mut self, head: Vec<TypeId>, tail: Option<TypePackId>) -> TypePackId {
        if head.is_empty() {
            if let Some(tail) = tail {
                tail
            } else {
                unsafe { (*self.builtin_types).emptyTypePack }
            }
        } else {
            let pack = TypePack { head, tail };
            let pack_var = TypePackVar {
                ty: TypePackVariant::TypePack(pack),
                persistent: false,
                owningArena: core::ptr::null_mut(),
            };

            unsafe { (*self.arena).add_type_pack_t(pack_var) }
        }
    }
}
