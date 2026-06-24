#[cfg(test)]
#[test]
fn compiler_debug_line_info_for() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from("\nfor\ni\nin\n1\n,\n2\n,\n3\ndo\nprint(i)\nend\n");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n5: LOADN R0 1\n7: LOADN R1 2\n9: LOADN R2 3\n9: FORGPREP R0 L1\n11: L0: GETIMPORT R5 1 [print]\n11: MOVE R6 R3\n11: CALL R5 1 0\n2: L1: FORGLOOP R0 L0 1\n13: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
