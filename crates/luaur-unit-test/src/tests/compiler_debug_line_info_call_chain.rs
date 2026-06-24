#[cfg(test)]
#[test]
fn compiler_debug_line_info_call_chain() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from("\nlocal Foo = ...\n\nFoo\n:Bar(1)\n:Baz(2)\n.Qux(3)\n");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n2: GETVARARGS R0 1\n5: LOADN R3 1\n5: NAMECALL R1 R0 K0 ['Bar']\n5: CALL R1 2 1\n6: LOADN R3 2\n6: NAMECALL R1 R1 K1 ['Baz']\n6: CALL R1 2 1\n7: GETTABLEKS R1 R1 K2 ['Qux']\n7: LOADN R2 3\n7: CALL R1 1 0\n8: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
