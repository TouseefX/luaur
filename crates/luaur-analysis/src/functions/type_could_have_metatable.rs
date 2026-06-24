use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

pub fn type_could_have_metatable(ty: TypeId) -> bool {
    let followed = unsafe { follow_type_id(ty) };

    unsafe {
        !get_type_id::<TableType>(followed).is_null()
            || !get_type_id::<ExternType>(followed).is_null()
            || !get_type_id::<MetatableType>(followed).is_null()
    }
}
