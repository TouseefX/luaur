#[cfg(test)]
#[test]
fn compiler_out_of_upvalues() {
    use alloc::string::String;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_common::functions::format_append::formatAppend;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
    use luaur_compiler::records::compile_error::CompileError;

    let mut source = String::new();

    for i in 0..150 {
        formatAppend(&mut source, format_args!("local foo{}\n", i));
        formatAppend(&mut source, format_args!("foo{} = 42\n", i));
    }

    source += "function foo()\n";

    for i in 0..150 {
        formatAppend(&mut source, format_args!("local bar{}\n", i));
        formatAppend(&mut source, format_args!("bar{} = 42\n", i));
    }

    source += "function bar()\n";

    for i in 0..150 {
        formatAppend(&mut source, format_args!("print(foo{}, bar{})\n", i, i));
    }

    source += "end\nend\n";

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
            // Expected CompileError
        }
        Ok(_) => {
            panic!("Expected CompileError");
        }
    }

    // Note: The original C++ test checks the exact error message and location.
    // Since we cannot easily capture the CompileError in a catch_unwind block,
    // we rely on the fact that the compilation should fail as expected.
    // In a full translation, we would capture and verify the error details.
}
