use crate::functions::follow_type::follow_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::singleton_type::SingletonType;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::type_id::TypeId;

pub fn get_table_match_tag(type_id: TypeId) -> Option<(Name, *const SingletonType)> {
    let ttv = get_table_type(type_id)?;

    for (name, prop) in &ttv.props {
        let prop_type_id = prop.read_ty.or(prop.write_ty).unwrap_or(core::ptr::null());
        let followed_type_id = unsafe { follow_type_id(prop_type_id) };
        let singleton_ptr = unsafe { get_type_id::<SingletonType>(followed_type_id) };
        if !singleton_ptr.is_null() {
            return Some((name.clone(), singleton_ptr));
        }
    }

    None
}
