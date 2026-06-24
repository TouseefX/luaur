use crate::functions::get_type_alt_j::get_type_id;
use crate::records::demoter::Demoter;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;

impl Demoter {
    pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
        let ftv = unsafe { get_type_id::<FreeType>(ty) };
        !ftv.is_null()
    }
}
