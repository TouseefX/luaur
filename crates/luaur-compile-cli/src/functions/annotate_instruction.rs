use core::ffi::c_void;

use alloc::string::String;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;

pub unsafe extern "C" fn annotate_instruction(
    context: *mut c_void,
    text: &mut String,
    fid: i32,
    instpos: i32,
) {
    let bcb = &*(context as *const BytecodeBuilder);
    bcb.annotate_instruction(text, fid as u32, instpos as u32);
}
