use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl Normalizer {
    pub fn union_saturated_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
        self.consume_fuel();

        let hftv = unsafe { get_type_id::<FunctionType>(here) };
        if hftv.is_null() {
            return None;
        }

        let tftv = unsafe { get_type_id::<FunctionType>(there) };
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

        let arg_types = self.union_of_type_packs(hftv_ref.arg_types, tftv_ref.arg_types)?;
        let ret_types = self.union_of_type_packs(hftv_ref.ret_types, tftv_ref.ret_types)?;

        let mut result = FunctionType::function_type_new(arg_types, ret_types, None, false);
        result.generics = hftv_ref.generics.clone();
        result.generic_packs = hftv_ref.generic_packs.clone();

        Some(unsafe { (*self.arena).add_type(result) })
    }
}
