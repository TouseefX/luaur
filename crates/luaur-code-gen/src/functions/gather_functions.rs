use alloc::vec::Vec;
use luaur_vm::records::proto::Proto;

use crate::functions::gather_functions_helper::gather_functions_helper;

pub fn gather_functions(
    results: &mut Vec<*mut Proto>,
    root: *mut Proto,
    flags: u32,
    has_native_functions: bool,
) {
    gather_functions_helper(results, root, flags, has_native_functions, true);
}
