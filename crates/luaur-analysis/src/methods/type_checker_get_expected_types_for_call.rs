use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::reduce_union::reduce_union;
use crate::records::demoter::Demoter;
use crate::records::function_type::FunctionType;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl TypeChecker {
    pub fn get_expected_types_for_call(
        &mut self,
        overloads: &Vec<TypeId>,
        argument_count: usize,
        self_call: bool,
    ) -> Vec<Option<TypeId>> {
        let mut expected_types: Vec<Option<TypeId>> = Vec::new();

        let mut assign_option = |index: usize, ty: TypeId| {
            if index == expected_types.len() {
                expected_types.push(Some(ty));
            } else if let Some(el) = expected_types.get_mut(index) {
                if let Some(existing) = *el {
                    let result = reduce_union(&[existing, ty]);
                    if result.is_empty() {
                        *el = Some(self.never_type);
                    } else if result.len() == 1 {
                        *el = Some(result[0]);
                    } else {
                        *el = Some(self.add_type(&UnionType { options: result }));
                    }
                } else {
                    *el = Some(ty);
                }
            }
        };

        for &overload in overloads {
            let ftv_ptr =
                unsafe { crate::functions::get_type_alt_j::get_type_id::<FunctionType>(overload) };
            if !ftv_ptr.is_null() {
                let ftv = unsafe { &*ftv_ptr };
                let (args_head, args_tail) = flatten_type_pack_id(ftv.arg_types);

                let start = if self_call { 1 } else { 0 };
                let mut index = 0;

                for i in start..args_head.len() {
                    assign_option(index, args_head[i]);
                    index += 1;
                }

                if let Some(tail) = args_tail {
                    let tail = unsafe { follow_type_pack_id(tail) };
                    let vtp_ptr = unsafe { get_type_pack_id::<VariadicTypePack>(tail) };
                    if !vtp_ptr.is_null() {
                        let vtp = unsafe { &*vtp_ptr };
                        while index < argument_count {
                            assign_option(index, vtp.ty);
                            index += 1;
                        }
                    }
                }
            }
        }

        let mut demoter = Demoter {
            arena: self.normalizer.arena,
            builtins: self.builtin_types,
        };
        demoter.demote(&mut expected_types);

        expected_types
    }
}
