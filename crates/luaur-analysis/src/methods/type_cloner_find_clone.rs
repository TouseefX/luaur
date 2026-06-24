use crate::enums::follow_option::FollowOption;
use crate::functions::follow_type_alt_c::follow_type_id_follow_option;
use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::type_id::TypeId;

impl TypeCloner {
    pub fn find_type_id(&self, ty: TypeId) -> Option<TypeId> {
        let ty = unsafe { follow_type_id_follow_option(ty, FollowOption::DisableLazyTypeThunks) };

        if let Some(it) = unsafe { (*self.types).get(&ty) } {
            return Some(*it);
        } else if unsafe { (*ty).persistent } && ty != self.force_ty {
            return Some(ty);
        }

        None
    }
}
