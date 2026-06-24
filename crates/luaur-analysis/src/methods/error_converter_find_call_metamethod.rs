use crate::functions::follow_type::follow_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::error_converter::ErrorConverter;
use crate::records::extern_type::ExternType;
use crate::records::metatable_type::MetatableType;
use crate::type_aliases::type_id::TypeId;

impl ErrorConverter {
    pub fn find_call_metamethod(&self, _type: TypeId) -> Option<TypeId> {
        let type_ = unsafe { follow_type_id(_type) };

        let mut metatable: Option<TypeId> = None;
        let metatable_ptr = unsafe { get_type_id::<MetatableType>(type_) };
        if !metatable_ptr.is_null() {
            metatable = Some(unsafe { (*metatable_ptr).metatable });
        } else {
            let extern_ptr = unsafe { get_type_id::<ExternType>(type_) };
            if !extern_ptr.is_null() {
                metatable = unsafe { (*extern_ptr).metatable };
            }
        }

        if metatable.is_none() {
            return None;
        }

        let unwrapped = unsafe { follow_type_id(metatable.unwrap()) };

        let any_ptr = unsafe { get_type_id::<AnyType>(unwrapped) };
        if !any_ptr.is_null() {
            return Some(unwrapped);
        }

        let mtt = get_table_type(unwrapped);
        if mtt.is_none() {
            return None;
        }

        let mtt = mtt.unwrap();
        if let Some(prop) = mtt.props.get("__call") {
            return prop.read_ty;
        }

        None
    }
}
