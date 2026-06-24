use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;

use alloc::string::String;
use alloc::vec::Vec;

fn read_var_int(data: *const core::ffi::c_char, offset: &mut usize) -> u32 {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;

    loop {
        let b = unsafe { *data.add(*offset) } as u8;
        *offset += 1;

        result |= ((b & 127) as u32) << shift;

        if (b & 128) == 0 {
            break;
        }

        shift += 7;
    }

    result
}

impl BytecodeCompilerFixture {
    pub fn extract_string_table(&mut self, bcb: &BytecodeBuilder) -> Vec<String> {
        let bytecode = bcb.get_bytecode();
        let data = bytecode.as_ptr() as *const core::ffi::c_char;

        let mut offset: usize = 2; // skip versions
        let mut result: Vec<String> = Vec::new();

        let strings_count = read_var_int(data, &mut offset);

        for _ in 0..strings_count {
            let str_len = read_var_int(data, &mut offset) as usize;

            let str_bytes =
                unsafe { core::slice::from_raw_parts(data.add(offset) as *const u8, str_len) };
            let s = String::from_utf8_lossy(str_bytes).into_owned();

            offset += str_len;
            result.push(s);
        }

        result
    }
}
