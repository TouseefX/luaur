use crate::records::extern_type::ExternType;
use crate::records::union_type::UnionType;
use crate::records::widen::Widen;
use crate::type_aliases::type_id::TypeId;

use crate::functions::get_type_alt_j::get_type_id;

impl Widen {
    pub fn widen_ignore_children(&self, ty: TypeId) -> bool {
        let et = unsafe { get_type_id::<ExternType>(ty) };
        if !et.is_null() {
            return true;
        }

        let ut = unsafe { get_type_id::<UnionType>(ty) };
        ut.is_null()
    }
}
