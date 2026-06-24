extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;

#[allow(non_snake_case)]
pub(crate) fn to_pointer_id<T>(ptr: *const T) -> String {
    (ptr as usize).to_string()
}
