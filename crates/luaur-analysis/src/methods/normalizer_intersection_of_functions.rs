use crate::records::function_type::FunctionType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Normalizer {
    pub fn intersection_of_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
        self.consume_fuel();

        let hftv: *const FunctionType =
            unsafe { crate::functions::get_type_alt_j::get_type_id::<FunctionType>(here) };
        if hftv.is_null() {
            return None;
        }
        let tftv: *const FunctionType =
            unsafe { crate::functions::get_type_alt_j::get_type_id::<FunctionType>(there) };
        if tftv.is_null() {
            return None;
        }

        let hftv_ref = unsafe { &*hftv };
        let tftv_ref = unsafe { &*tftv };

        if hftv_ref.generics != tftv_ref.generics {
            return None;
        }
        if hftv_ref.generic_packs != tftv_ref.generic_packs {
            return None;
        }

        let mut arg_types: TypePackId = unsafe { core::ptr::null() };
        let mut ret_types: TypePackId = unsafe { core::ptr::null() };

        if hftv_ref.ret_types == tftv_ref.ret_types {
            let arg_types_opt = self.union_of_type_packs(hftv_ref.arg_types, tftv_ref.arg_types);
            if arg_types_opt.is_none() {
                return None;
            }
            arg_types = arg_types_opt.unwrap();
            ret_types = hftv_ref.ret_types;
        } else if hftv_ref.arg_types == tftv_ref.arg_types {
            let ret_types_opt =
                self.intersection_of_type_packs_internal(hftv_ref.arg_types, tftv_ref.arg_types);
            if ret_types_opt.is_none() {
                return None;
            }
            arg_types = hftv_ref.arg_types;
            ret_types = ret_types_opt.unwrap();
        } else {
            return None;
        }

        if arg_types == hftv_ref.arg_types && ret_types == hftv_ref.ret_types {
            return Some(here);
        }
        if arg_types == tftv_ref.arg_types && ret_types == tftv_ref.ret_types {
            return Some(there);
        }

        let mut result = FunctionType::function_type_new(arg_types, ret_types, None, false);
        result.generics = hftv_ref.generics.clone();
        result.generic_packs = hftv_ref.generic_packs.clone();

        Some(unsafe { (*self.arena).add_type(result) })
    }
}
