extern crate alloc;

use crate::enums::ir_const_kind::IrConstKind;
use crate::functions::append::append;
use crate::functions::append_vm_constant::append_vm_constant;
use crate::functions::format_g::format_g;
use crate::functions::get_tag_name::get_tag_name;
use crate::records::ir_const::IrConst;
use alloc::string::String;
use luaur_vm::type_aliases::proto::Proto;

pub fn to_string(result: &mut String, proto: *mut Proto, constant: IrConst) {
    unsafe {
        match constant.kind {
            IrConstKind::Int => append(result, format_args!("{}i", constant.value.value_int)),
            IrConstKind::Int64 => append(
                result,
                format_args!("{}i", constant.value.value_int64 as i64),
            ),
            IrConstKind::Uint => append(result, format_args!("{}u", constant.value.value_uint)),
            IrConstKind::Double => {
                let d = constant.value.value_double;
                if d != d {
                    append(result, format_args!("nan"));
                } else {
                    // C++ uses printf "%.17g"; Rust's `{}` prints the shortest
                    // round-tripping form (e.g. `0.4` vs `0.40000000000000002`).
                    result.push_str(&format_g(d, 17));
                }
            }
            IrConstKind::Tag => result.push_str(get_tag_name(constant.value.value_tag)),
            IrConstKind::Import => {
                append(result, format_args!("{}u", constant.value.value_uint));

                if !proto.is_null() {
                    append(result, format_args!(" ("));

                    let value_uint = constant.value.value_uint;
                    let count = (value_uint >> 30) as i32;
                    let id0 = if count > 0 {
                        ((value_uint >> 20) & 1023) as i32
                    } else {
                        -1
                    };
                    let id1 = if count > 1 {
                        ((value_uint >> 10) & 1023) as i32
                    } else {
                        -1
                    };
                    let id2 = if count > 2 {
                        (value_uint & 1023) as i32
                    } else {
                        -1
                    };

                    if id0 != -1 {
                        append_vm_constant(result, proto, id0);
                    }

                    if id1 != -1 {
                        append(result, format_args!("."));
                        append_vm_constant(result, proto, id1);
                    }

                    if id2 != -1 {
                        append(result, format_args!("."));
                        append_vm_constant(result, proto, id2);
                    }

                    append(result, format_args!(")"));
                }
            }
        }
    }
}
