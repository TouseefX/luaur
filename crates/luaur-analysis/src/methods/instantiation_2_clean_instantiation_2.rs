use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::free_type::FreeType;
use crate::records::generic_type::GenericType;
use crate::records::instantiation_2::Instantiation2;
use crate::records::never_type::NeverType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Instantiation2 {
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        LUAU_ASSERT!(!self.subtyping.is_null() && !self.scope.is_null());
        let generic = unsafe { get_type_id::<GenericType>(ty) };
        LUAU_ASSERT!(!generic.is_null());

        let subst_ty = unsafe {
            follow_type_id(
                *self
                    .generic_substitutions
                    .find(&ty)
                    .expect("TypeId not found in generic_substitutions"),
            )
        };
        let ft = unsafe { get_type_id::<FreeType>(subst_ty) };
        LUAU_ASSERT!(!ft.is_null());

        let lower_bound = unsafe { (*ft).lower_bound };
        let upper_bound = unsafe { (*ft).upper_bound };

        let res = if unsafe { !get_type_id::<NeverType>(follow_type_id(lower_bound)).is_null() } {
            upper_bound
        } else if unsafe { !get_type_id::<UnknownType>(follow_type_id(upper_bound)).is_null() } {
            lower_bound
        } else {
            let r = unsafe {
                (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
                    lower_bound,
                    upper_bound,
                    self.scope,
                )
            };
            if r.is_subtype {
                lower_bound
            } else {
                upper_bound
            }
        };

        self.base.dont_traverse_into_type_id(res);
        res
    }
}
