use crate::records::builtin_types::BuiltinTypes;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use core::ffi::c_char;
use core::ptr::NonNull;

pub fn parse_pattern_string(
    builtin_types: NonNull<BuiltinTypes>,
    data: *const c_char,
    size: usize,
) -> Vec<TypeId> {
    let builtin_types = unsafe { builtin_types.as_ref() };
    let data = unsafe { core::slice::from_raw_parts(data as *const u8, size) };

    let mut result = Vec::new();
    let mut depth = 0;
    let mut parsing_set = false;

    let mut i = 0;
    while i < size {
        let b = data[i];
        if b == b'%' {
            i += 1;
            if !parsing_set && i < size && data[i] == b'b' {
                i += 2;
            }
        } else if !parsing_set && b == b'[' {
            parsing_set = true;
            if i + 1 < size && data[i + 1] == b']' {
                i += 1;
            }
        } else if parsing_set && b == b']' {
            parsing_set = false;
        } else if b == b'(' {
            if !parsing_set {
                if i + 1 < size && data[i + 1] == b')' {
                    i += 1;
                    result.push(builtin_types.optionalNumberType);
                } else {
                    depth += 1;
                    result.push(builtin_types.optionalStringType);
                }
            }
        } else if b == b')' {
            if !parsing_set {
                depth -= 1;
                if depth < 0 {
                    break;
                }
            }
        }
        i += 1;
    }

    if depth != 0 || parsing_set {
        return Vec::new();
    }

    if result.is_empty() {
        result.push(builtin_types.optionalStringType);
    }

    result
}
