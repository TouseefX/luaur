use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::demoter::Demoter;
use crate::records::free_type_pack::FreeTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl Demoter {
    pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
        let ftp = unsafe { get_type_pack_id::<FreeTypePack>(tp) };
        !ftp.is_null()
    }
}
