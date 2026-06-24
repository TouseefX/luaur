use crate::functions::follow_type::follow;
use crate::records::generic_bounds::GenericBounds;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::type_aliases::type_id::TypeId;
use luaur_common::LUAU_ASSERT;

impl SubtypingEnvironment {
    pub fn get_mapped_type_bounds(
        &mut self,
        ty: TypeId,
        ice_reporter: *mut InternalErrorReporter,
    ) -> &mut GenericBounds {
        let ty = unsafe { follow(ty) };
        if let Some(bounds) =
            crate::methods::subtyping_bind_generic::dense_hash_map_find_mut_no_default(
                &mut self.mapped_generics,
                &ty,
            )
        {
            if !bounds.is_empty() {
                return bounds.last_mut().unwrap();
            }
        }

        if !self.parent.is_null() {
            return unsafe { (*self.parent).get_mapped_type_bounds(ty, ice_reporter) };
        }

        LUAU_ASSERT!(false);
        unsafe {
            (*ice_reporter)
                .ice_string("Trying to access bounds for a type with no in-scope bounds");
        }
        unreachable!()
    }
}
