use alloc::string::String;

use crate::functions::get_bytecode_type_name::get_bytecode_type_name;
use crate::records::bytecode_types::BytecodeTypes;

const LBC_TYPE_OPTIONAL_BIT: u8 = 0x80;

pub(crate) fn to_string_string_bytecode_types_c_char(
    result: &mut String,
    bc_types: &BytecodeTypes,
    userdata_types: *const *const core::ffi::c_char,
) {
    let optional_suffix = |t: u8| {
        if (t & LBC_TYPE_OPTIONAL_BIT) != 0 {
            "?"
        } else {
            ""
        }
    };

    unsafe {
        let result_ty = get_bytecode_type_name(bc_types.result, userdata_types);
        let a_ty = get_bytecode_type_name(bc_types.a, userdata_types);
        let b_ty = get_bytecode_type_name(bc_types.b, userdata_types);

        let result_ty_str = core::ffi::CStr::from_ptr(result_ty).to_string_lossy();
        let a_ty_str = core::ffi::CStr::from_ptr(a_ty).to_string_lossy();
        let b_ty_str = core::ffi::CStr::from_ptr(b_ty).to_string_lossy();

        result.push_str(&result_ty_str);
        result.push_str(optional_suffix(bc_types.result));

        result.push_str(" <- ");

        result.push_str(&a_ty_str);
        result.push_str(optional_suffix(bc_types.a));

        result.push_str(", ");

        result.push_str(&b_ty_str);
        result.push_str(optional_suffix(bc_types.b));

        if bc_types.c != crate::records::bytecode_types::LBC_TYPE_ANY {
            let c_ty = get_bytecode_type_name(bc_types.c, userdata_types);
            let c_ty_str = core::ffi::CStr::from_ptr(c_ty).to_string_lossy();
            result.push_str(", ");
            result.push_str(&c_ty_str);
            result.push_str(optional_suffix(bc_types.c));
        }
    }
}
