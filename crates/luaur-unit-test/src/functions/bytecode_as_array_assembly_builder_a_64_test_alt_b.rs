use alloc::string::String;

use luaur_common::functions::format_append::formatAppend;

pub fn bytecode_as_array_vector_u32(code: &[u32]) -> String {
    let mut result = String::from("{");

    for i in 0..code.len() {
        let sep = if i == 0 { "" } else { ", " };
        formatAppend(&mut result, format_args!("{}0x{:08x}", sep, code[i]));
    }

    result.push('}');
    result
}
