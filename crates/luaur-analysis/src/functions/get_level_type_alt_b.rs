use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn get_level_type_pack_id(tp: TypePackId) -> Option<TypeLevel> {
    let tp = unsafe { follow_type_pack_id(tp) };

    let ftv = unsafe { get_type_pack_id::<FreeTypePack>(tp) };

    if !ftv.is_null() {
        Some(unsafe { (*ftv).level })
    } else {
        None
    }
}
