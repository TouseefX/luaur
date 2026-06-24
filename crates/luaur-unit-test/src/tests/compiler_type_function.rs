#[cfg(test)]
#[test]
fn compiler_type_function() {
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
    use luaur_compiler::records::compile_options::CompileOptions;

    let mut bcb = BytecodeBuilder::new(None);
    let options = CompileOptions::default();
    let parse_options = ParseOptions::default();

    let source = "type function a() return types.any end";
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source.to_string(),
        &options,
        &parse_options,
    );
}
