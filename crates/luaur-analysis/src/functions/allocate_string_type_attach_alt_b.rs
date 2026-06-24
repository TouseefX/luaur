extern crate alloc;

use alloc::format;
use core::ffi::c_char;
use luaur_ast::records::allocator::Allocator;

pub fn allocate_string_luau_allocator_c_char_data(
    allocator: &mut Allocator,
    format_str: &str,
    args: core::fmt::Arguments<'_>,
) -> *mut c_char {
    let formatted = format!("{}", args);
    let len = formatted.len();
    let result = allocator.allocate(len + 1) as *mut c_char;

    unsafe {
        core::ptr::copy_nonoverlapping(formatted.as_ptr() as *const c_char, result, len);
        *result.add(len) = 0;
    }

    result
}
