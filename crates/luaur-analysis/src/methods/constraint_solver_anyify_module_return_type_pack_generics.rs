use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::generic_type::GenericType;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl ConstraintSolver {
    pub fn anyify_module_return_type_pack_generics(&mut self, tp: TypePackId) -> TypePackId {
        let tp = unsafe { follow_type_pack_id(tp) };

        if let Some(vtp) = unsafe { get_type_pack_id::<VariadicTypePack>(tp).as_ref() } {
            let ty = unsafe { follow_type_id(vtp.ty) };
            return if !unsafe { get_type_id::<GenericType>(ty) }.is_null() {
                unsafe { (*self.builtin_types).anyTypePack }
            } else {
                tp
            };
        }

        if unsafe { get_type_pack_id::<TypePack>(tp) }.is_null() {
            return tp;
        }

        let mut result_types = Vec::new();
        let mut it = begin_type_pack_id(tp);
        let end_it = end_type_pack_id(tp);

        while it.operator_ne(&end_it) {
            let ty = unsafe { follow_type_id(*it.operator_deref()) };
            result_types.push(if !unsafe { get_type_id::<GenericType>(ty) }.is_null() {
                unsafe { (*self.builtin_types).anyType }
            } else {
                ty
            });
            it.operator_inc();
        }

        let result_tail = if let Some(tail) = it.tail() {
            Some(self.anyify_module_return_type_pack_generics(tail))
        } else {
            None
        };

        unsafe {
            (*self.arena)
                .add_type_pack_vector_type_id_optional_type_pack_id(result_types, result_tail)
        }
    }
}
