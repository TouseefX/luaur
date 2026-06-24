use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::try_unify_with_any::try_unify_with_any;
use crate::records::any_type::AnyType;
use crate::records::extern_type::ExternType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::unifier::Unifier;
use crate::records::unknown_type::UnknownType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Unifier {
    pub fn try_unify_with_any_type_id_type_id(&mut self, sub_ty: TypeId, any_ty: TypeId) {
        LUAU_ASSERT!(
            !unsafe { get_type_id::<AnyType>(any_ty) }.is_null()
                || !unsafe { get_type_id::<ErrorType>(any_ty) }.is_null()
                || !unsafe { get_type_id::<UnknownType>(any_ty) }.is_null()
                || !unsafe { get_type_id::<NeverType>(any_ty) }.is_null()
        );

        if !unsafe { get_type_id::<PrimitiveType>(sub_ty) }.is_null()
            || !unsafe { get_type_id::<AnyType>(sub_ty) }.is_null()
            || !unsafe { get_type_id::<ExternType>(sub_ty) }.is_null()
        {
            return;
        }

        let any_tp = unsafe {
            (*self.types).add_type_pack_t(VariadicTypePack {
                ty: any_ty,
                hidden: false,
            })
        };
        let mut queue: Vec<TypeId> = alloc::vec![sub_ty];

        unsafe {
            let shared_state = &mut *self.shared_state;
            shared_state.temp_seen_ty.clear();
            shared_state.temp_seen_tp.clear();
            let seen_ty = &mut shared_state.temp_seen_ty as *mut _;
            let seen_tp = &mut shared_state.temp_seen_tp as *mut _;

            try_unify_with_any(
                &mut queue,
                self,
                &mut *seen_ty,
                &mut *seen_tp,
                self.types,
                any_ty,
                any_tp,
            );
        }
    }
}
