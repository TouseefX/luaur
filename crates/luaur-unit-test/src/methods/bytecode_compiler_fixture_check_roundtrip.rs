use crate::functions::extract_code::extract_code;
use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
use alloc::vec::Vec;
use luaur_bytecode::functions::from_function_bytecode::from_function_bytecode;
use luaur_bytecode::functions::to_function_bytecode_bytecode_graph_alt_b::to_function_bytecode_comp_time_bc_function;

impl BytecodeCompilerFixture {
    pub fn check_roundtrip(&mut self, snippet: &str) {
        for opt_level in 0..=2 {
            let (function_bytecode, strings) = self
                .get_function_bytecode(snippet, opt_level)
                .expect("expected bytecode for roundtrip snippet");

            let mut table: Vec<&[u8]> = strings.iter().map(|s| s.as_bytes()).collect();
            let mut function = from_function_bytecode(function_bytecode.clone(), &mut table)
                .expect("expected function bytecode to deserialize");

            let orig = extract_code(&function_bytecode);
            let dumped = extract_code(&to_function_bytecode_comp_time_bc_function(&mut function));
            assert_eq!(orig, dumped);
        }
    }
}
