use crate::records::builtin_types::BuiltinTypes;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use core::ffi::c_char;
use core::ptr::NonNull;
use core::slice;

pub fn parse_format_string(
    builtin_types: NonNull<BuiltinTypes>,
    data: *const c_char,
    size: usize,
) -> Vec<TypeId> {
    let options = b"cdiouxXeEfgGqs*";
    let data_slice = unsafe { slice::from_raw_parts(data as *const u8, size) };
    let mut result = Vec::new();
    let builtin_types = unsafe { builtin_types.as_ref() };

    let mut i = 0;
    while i < size {
        if data_slice[i] == b'%' {
            i += 1;

            if i < size && data_slice[i] == b'%' {
                i += 1;
                continue;
            }

            while i < size
                && !(data_slice[i] > 0
                    && (data_slice[i].is_ascii_alphabetic() || data_slice[i] == b'*'))
            {
                i += 1;
            }

            if i == size {
                break;
            }

            let c = data_slice[i];
            if c == b'q' || c == b's' {
                result.push(builtin_types.stringType);
            } else if c == b'*' {
                result.push(builtin_types.unknownType);
            } else if options.contains(&c) {
                result.push(builtin_types.numberType);
            } else {
                result.push(builtin_types.error_recovery_type(builtin_types.anyType));
            }
        }
        i += 1;
    }

    result
}
