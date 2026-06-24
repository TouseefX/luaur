#[cfg(test)]
#[test]
fn compiler_vector_fast_call3() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let source = String::from("local a, b, c = ...\nreturn Vector3.new(a, b, c)");
    let mut options = luaur_compiler::records::compile_options::CompileOptions::default();
    options.vector_lib = c"Vector3".as_ptr();
    options.vector_ctor = c"new".as_ptr();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\nGETVARARGS R0 3\nFASTCALL3 54 R0 R1 R2 L0\nMOVE R4 R0\nMOVE R5 R1\nMOVE R6 R2\nGETIMPORT R3 2 [Vector3.new]\nCALL R3 3 -1\nL0: RETURN R3 -1\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
