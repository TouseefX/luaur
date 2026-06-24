use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::negation_type::NegationType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn mk_negation(&self, ty: TypeId) -> TypeId {
        let builtin_types = unsafe { &*self.builtin_types };
        let arena = unsafe { &mut *self.arena.cast_mut() };
        if ty == builtin_types.truthyType {
            builtin_types.falsyType
        } else if ty == builtin_types.falsyType {
            builtin_types.truthyType
        } else if let Some(ntv) = unsafe { get_type_id::<NegationType>(ty).as_ref() } {
            unsafe { follow_type_id(ntv.ty) }
        } else {
            arena.add_type(NegationType { ty })
        }
    }
}
