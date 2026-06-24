use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::reduce_union::reduce_union;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

impl ConstraintGenerator {
    pub fn get_expected_call_types_for_function_overloads(
        &mut self,
        fn_type: TypeId,
    ) -> Vec<Option<TypeId>> {
        let mut fun_tys: Vec<TypeId> = Vec::new();
        let followed_fn_type = unsafe { follow_type_id(fn_type) };
        let ity_ptr = unsafe { get_type_id::<IntersectionType>(followed_fn_type) };
        if !ity_ptr.is_null() {
            let ity = unsafe { &*ity_ptr };
            for &intersection_component in &ity.parts {
                fun_tys.push(intersection_component);
            }
        }

        let mut expected_types: Vec<Option<TypeId>> = Vec::new();

        let mut assign_option = |index: usize, ty: TypeId| {
            if index == expected_types.len() {
                expected_types.push(Some(ty));
            } else if let Some(el) = expected_types.get_mut(index) {
                if let Some(existing) = *el {
                    let result = reduce_union(&[existing, ty]);
                    if result.is_empty() {
                        *el = Some(unsafe { (*self.builtin_types).neverType });
                    } else if result.len() == 1 {
                        *el = Some(result[0]);
                    } else {
                        *el = Some(self.make_union_vector_type_id(result));
                    }
                } else {
                    *el = Some(ty);
                }
            }
        };

        for &overload in &fun_tys {
            let followed_overload = unsafe { follow_type_id(overload) };
            let ftv_ptr = unsafe { get_type_id::<FunctionType>(followed_overload) };
            if !ftv_ptr.is_null() {
                let ftv = unsafe { &*ftv_ptr };
                let (args_head, args_tail) = flatten_type_pack_id(ftv.arg_types);

                let start = if ftv.has_self { 1 } else { 0 };
                let mut index = 0;

                for i in start..args_head.len() {
                    assign_option(index, args_head[i]);
                    index += 1;
                }

                if let Some(args_tail_id) = args_tail {
                    let tail = unsafe { follow_type_pack::follow(args_tail_id) };
                    let vtp_ptr = unsafe { get_type_pack_id::<VariadicTypePack>(tail) };
                    if !vtp_ptr.is_null() {
                        let vtp = unsafe { &*vtp_ptr };
                        while index < fun_tys.len() {
                            assign_option(index, vtp.ty);
                            index += 1;
                        }
                    }
                }
            }
        }

        expected_types
    }
}
