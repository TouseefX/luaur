use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::property_type::Property;
use crate::type_aliases::name_type::Name;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn lookup_extern_type_prop(cls: &ExternType, name: &Name) -> *const Property {
    let mut cls_ptr = cls as *const ExternType;

    while !cls_ptr.is_null() {
        let cls_ref = unsafe { &*cls_ptr };

        if let Some(prop) = cls_ref.props.get(name) {
            return prop as *const Property;
        }

        match cls_ref.parent {
            Some(parent_id) => {
                let parent_ptr = unsafe { get_type_id::<ExternType>(parent_id) };
                cls_ptr = parent_ptr as *const ExternType;
                LUAU_ASSERT!(!cls_ptr.is_null());
            }
            None => return core::ptr::null(),
        }
    }

    core::ptr::null()
}
