use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::free_type::FreeType;
use crate::records::generic_type::GenericType;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;

pub fn can_suggest_inferred_type(ty: TypeId) -> bool {
    unsafe {
        let ty = follow_type_id(ty);

        // No point in suggesting 'any', invalid to suggest others
        if !get_type_id::<AnyType>(ty).is_null()
            || !get_type_id::<ErrorType>(ty).is_null()
            || !get_type_id::<GenericType>(ty).is_null()
            || !get_type_id::<FreeType>(ty).is_null()
        {
            return false;
        }

        // No syntax for unnamed tables with a metatable
        if !get_type_id::<MetatableType>(ty).is_null() {
            return false;
        }

        let ttv_ptr = get_type_id::<TableType>(ty);
        if !ttv_ptr.is_null() {
            let ttv = &*ttv_ptr;
            if ttv.name.is_some() {
                return true;
            }

            if ttv.synthetic_name.is_some() {
                return false;
            }
        }

        // We might still have a type with cycles or one that is too long, we'll check that later
        true
    }
}
