#[cfg(test)]
#[test]
fn compiler_lots_of_indexers() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let source = String::from(
        "\nfunction u(t)for t in s(t[l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l][l],l)do end\nend\n",
    );

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

    assert!(result.is_err(), "Expected exception");
    let err = result.unwrap_err();
    let msg = err
        .downcast_ref::<luaur_compiler::records::compile_error::CompileError>()
        .map(|e| alloc::format!("{e}"))
        .or_else(|| err.downcast_ref::<String>().cloned())
        .or_else(|| {
            err.downcast_ref::<&'static str>()
                .map(|s| alloc::string::ToString::to_string(s))
        })
        .unwrap_or_default();
    assert_eq!(
        msg,
        "Out of registers when trying to allocate 1 registers: exceeded limit 255"
    );
}
