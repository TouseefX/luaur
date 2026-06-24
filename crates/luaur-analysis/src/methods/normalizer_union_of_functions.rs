use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Normalizer {
    pub fn union_of_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
        self.consume_fuel();

        unsafe {
            if !get_type_id::<ErrorType>(here).is_null() {
                return Some(here);
            }

            if !get_type_id::<ErrorType>(there).is_null() {
                return Some(there);
            }

            let hftv = get_type_id::<FunctionType>(here);
            LUAU_ASSERT!(!hftv.is_null());
            let tftv = get_type_id::<FunctionType>(there);
            LUAU_ASSERT!(!tftv.is_null());

            let h_generics = (*hftv).generics.clone();
            let t_generics = (*tftv).generics.clone();
            if h_generics != t_generics {
                return None;
            }

            let h_generic_packs = (*hftv).generic_packs.clone();
            let t_generic_packs = (*tftv).generic_packs.clone();
            if h_generic_packs != t_generic_packs {
                return None;
            }

            let arg_types =
                self.intersection_of_type_packs_internal((*hftv).arg_types, (*tftv).arg_types);
            if arg_types.is_none() {
                return None;
            }

            let ret_types = self.union_of_type_packs((*hftv).ret_types, (*tftv).ret_types);
            if ret_types.is_none() {
                return None;
            }

            let arg_types_val = arg_types.unwrap();
            let ret_types_val = ret_types.unwrap();

            if arg_types_val == (*hftv).arg_types && ret_types_val == (*hftv).ret_types {
                return Some(here);
            }

            if arg_types_val == (*tftv).arg_types && ret_types_val == (*tftv).ret_types {
                return Some(there);
            }

            let mut result =
                FunctionType::function_type_new(arg_types_val, ret_types_val, None, false);
            result.generics = h_generics;
            result.generic_packs = h_generic_packs;

            Some((*self.arena).add_type(result))
        }
    }
}
