use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_utils::get_optional_ty;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::replace_generics::ReplaceGenerics;
use crate::type_aliases::type_id::TypeId;

impl ReplaceGenerics {
    pub fn ignore_children(&self, ty: TypeId) -> bool {
        let ftv = unsafe { get_type_id::<FunctionType>(ty) };
        if !ftv.is_null() {
            let ftv_ref = unsafe { &*ftv };
            if ftv_ref.has_no_free_or_generic_types {
                return true;
            }

            return (!self.generics.is_empty() || !self.generic_packs.is_empty())
                && (ftv_ref.generics == self.generics)
                && (ftv_ref.generic_packs == self.generic_packs);
        }

        let et = unsafe { get_optional_ty::<ExternType, TypeId>(Some(ty)) };
        if !et.is_null() {
            return true;
        }

        false
    }
}
