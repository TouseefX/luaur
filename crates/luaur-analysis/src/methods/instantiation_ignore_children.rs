use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::instantiation::Instantiation;
use crate::type_aliases::type_id::TypeId;

use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_utils::get_optional_ty;

impl Instantiation {
    pub fn ignore_children(&self, ty: TypeId) -> bool {
        let ft = unsafe { get_type_id::<FunctionType>(ty) };
        if !ft.is_null() {
            return true;
        }

        let et = unsafe { get_optional_ty::<ExternType, TypeId>(Some(ty)) };
        !et.is_null()
    }
}
