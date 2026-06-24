#[cfg(test)]
#[test]
fn compiler_out_of_locals() {
    use alloc::string::String;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
    use luaur_compiler::records::compile_options::CompileOptions;

    let mut source = String::new();

    for i in 0..200 {
        let mut temp = String::from("local foo");
        temp.push_str(&i.to_string());
        temp.push('\n');
        source.push_str(&temp);
    }

    source.push_str("local bar\n");

    let mut options = CompileOptions::default();
    options.debug_level = 2;

    let parse_options = ParseOptions::default();

    let mut bcb = BytecodeBuilder::new(None);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        compile_or_throw_bytecode_builder_string_compile_options_parse_options(
            &mut bcb,
            &source,
            &options,
            &parse_options,
        );
    }));

    assert!(result.is_err(), "Expected CompileError");

    let err = result.unwrap_err();
    // The panic payload is the CompileError object (panic_any), not a String.
    let msg = err
        .downcast_ref::<luaur_compiler::records::compile_error::CompileError>()
        .map(|e| alloc::format!("{e}"))
        .or_else(|| err.downcast_ref::<String>().cloned())
        .or_else(|| {
            err.downcast_ref::<&'static str>()
                .map(|s| alloc::string::ToString::to_string(s))
        })
        .unwrap_or_default();
    assert!(msg.contains("Out of local registers when trying to allocate bar: exceeded limit 200"));
}
