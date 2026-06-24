use crate::functions::generate_name::generate_name;
use crate::records::generic_type::GenericType;
use crate::type_aliases::synthetic_names::SyntheticNames;
use alloc::string::ToString;
use core::ffi::c_char;
use luaur_ast::records::allocator::Allocator;

pub fn get_name_allocator_synthetic_names_generic_type(
    allocator: &mut Allocator,
    synthetic_names: &mut SyntheticNames,
    gen: &GenericType,
) -> *mut c_char {
    let s = synthetic_names.size();
    let n_ptr =
        synthetic_names.get_or_insert(gen as *const GenericType as *const core::ffi::c_void);

    unsafe {
        if (*n_ptr).is_null() {
            let str = if gen.explicit_name {
                gen.name.to_string()
            } else {
                generate_name(s)
            };

            let size = str.len();
            let n = allocator.allocate(size + 1) as *mut c_char;
            core::ptr::copy_nonoverlapping(str.as_ptr() as *const c_char, n, size);
            *n.add(size) = 0;
            *n_ptr = n;
        }
        *n_ptr
    }
}
