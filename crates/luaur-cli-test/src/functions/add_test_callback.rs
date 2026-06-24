use crate::functions::get_register_callbacks::get_register_callbacks;
use crate::type_aliases::register_callback::RegisterCallback;
use std::collections::HashSet;

#[allow(non_snake_case)]
pub fn add_test_callback(cb: RegisterCallback) -> i32 {
    get_register_callbacks().insert(cb);
    0
}
