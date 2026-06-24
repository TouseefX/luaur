use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::generic_type::GenericType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl TypeChecker {
    pub fn anyify_module_return_type_pack_generics(&mut self, tp: TypePackId) -> TypePackId {
        let tp = unsafe { follow_type_pack_id(tp) };

        if let Some(vtp) =
            unsafe { crate::functions::get_type_pack::get::<VariadicTypePack>(tp).as_ref() }
        {
            let ty = unsafe { follow_type_id(vtp.ty) };
            return if unsafe { crate::functions::get_type_alt_j::get::<GenericType>(ty).as_ref() }
                .is_some()
            {
                self.any_type_pack
            } else {
                tp
            };
        }

        if unsafe { crate::functions::get_type_pack::get::<TypePack>(tp).as_ref() }.is_none() {
            return tp;
        }

        let mut result_types = Vec::new();
        let mut result_tail = None;

        let mut it = begin_type_pack_id(tp);
        let end_it = end_type_pack_id(tp);

        while it.operator_ne(&end_it) {
            let ty = unsafe { follow_type_id(*it.operator_deref()) };
            result_types.push(
                if unsafe { crate::functions::get_type_alt_j::get::<GenericType>(ty).as_ref() }
                    .is_some()
                {
                    self.any_type
                } else {
                    ty
                },
            );
            it.operator_inc();
        }

        if let Some(tail) = it.tail() {
            result_tail = Some(self.anyify_module_return_type_pack_generics(tail));
        }

        self.add_type_pack_vector_type_id_optional_type_pack_id(&result_types, result_tail)
    }
}
