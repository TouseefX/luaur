use crate::functions::get_type_alt_j::get_type_id;
use crate::records::demoter::Demoter;
use crate::records::extern_type::ExternType;
use crate::type_aliases::type_id::TypeId;

impl Demoter {
    pub fn ignore_children(&mut self, ty: TypeId) -> bool {
        let et = unsafe { get_type_id::<ExternType>(ty) };
        !et.is_null()
    }
}
