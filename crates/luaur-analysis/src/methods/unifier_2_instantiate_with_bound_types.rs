use crate::records::replacer::Replacer;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;

impl Unifier2 {
    pub fn instantiate_with_bound_types(&mut self, ty: TypeId) -> TypeId {
        let mut r = Replacer::replacer(
            self.arena.as_ptr(),
            NonNull::from(&mut self.generic_substitutions).as_ptr(),
            NonNull::from(&mut self.generic_pack_substitutions).as_ptr(),
        );
        if let Some(new_ty) = r.substitute_type_id(ty) {
            return new_ty;
        }
        ty
    }
}
