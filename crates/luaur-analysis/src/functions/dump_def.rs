extern crate alloc;

use crate::type_aliases::definition::Definition;
use alloc::string::String;

pub fn dump_def(def: *mut Definition) -> String {
    unsafe {
        if !def.is_null() {
            return (*def).versioned_name();
        }
    }
    String::from("?")
}
