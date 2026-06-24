#[cfg(test)]
#[test]
fn compiler_debug_line_info_call() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from("\nlocal Foo = ...\n\nFoo:Bar(\n    1,\n    2,\n    3)\n");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n2: GETVARARGS R0 1\n5: LOADN R3 1\n6: LOADN R4 2\n7: LOADN R5 3\n4: NAMECALL R1 R0 K0 ['Bar']\n4: CALL R1 4 0\n8: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
