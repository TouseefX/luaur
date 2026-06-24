use crate::functions::get_type_alt_j::get_type_id;
use crate::records::anyification::Anyification;
use crate::records::extern_type::ExternType;
use crate::type_aliases::type_id::TypeId;

impl Anyification {
    pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
        let et = unsafe { get_type_id::<ExternType>(ty) };
        if !et.is_null() {
            return true;
        }

        unsafe { (*ty).persistent }
    }
}
