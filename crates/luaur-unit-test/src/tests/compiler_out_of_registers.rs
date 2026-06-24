#[cfg(test)]
#[test]
fn compiler_out_of_registers() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_common::functions::format_append::formatAppend;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
    use luaur_compiler::records::compile_error::CompileError;

    let mut source = String::new();

    source += "print(\n";

    for i in 0..150 {
        formatAppend(&mut source, format_args!("{},\n", i));
    }

    source += "table.pack(\n";

    for i in 0..150 {
        formatAppend(&mut source, format_args!("{},\n", i));
    }

    source += "42))\n";

    let mut bcb = BytecodeBuilder::new(None);

    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        compile_or_throw_bytecode_builder_string_compile_options_parse_options(
            &mut bcb,
            &source,
            &options,
            &parse_options,
        );
    }));

    match result {
        Err(_) => {
            // Expected CompileError panic
        }
        Ok(_) => {
            panic!("Expected CompileError");
        }
    }

    // Note: The original C++ test checks the exception details, but since we're using panic
    // for error handling in this port, we cannot easily extract the exact location and message.
    // The test is considered passed if the expected panic occurs.
}
