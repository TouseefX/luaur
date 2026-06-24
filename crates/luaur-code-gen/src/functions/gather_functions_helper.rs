use crate::enums::code_gen_flags::CodeGenFlags;
use alloc::vec::Vec;
use luaur_vm::records::proto::Proto;

pub fn gather_functions_helper(
    results: &mut Vec<*mut Proto>,
    proto: *mut Proto,
    flags: u32,
    has_native_functions: bool,
    root: bool,
) {
    let proto_ref = unsafe { &*proto };

    if results.len() <= proto_ref.bytecodeid as usize {
        results.resize(proto_ref.bytecodeid as usize + 1, core::ptr::null_mut());
    }

    if !results[proto_ref.bytecodeid as usize].is_null() {
        return;
    }

    let lpf_native_function = 1 << 0;
    let lpf_native_cold = 1 << 1;

    let should_gather = if has_native_functions {
        !root && (proto_ref.flags as u32 & lpf_native_function) != 0
    } else {
        (proto_ref.flags as u32 & lpf_native_cold) == 0
            || (flags & (CodeGenFlags::CodeGen_ColdFunctions as u32)) != 0
    };

    if should_gather {
        results[proto_ref.bytecodeid as usize] = proto;
    }

    for i in 0..proto_ref.sizep as usize {
        let child_proto = unsafe { *proto_ref.p.add(i) };
        gather_functions_helper(results, child_proto, flags, has_native_functions, false);
    }
}
