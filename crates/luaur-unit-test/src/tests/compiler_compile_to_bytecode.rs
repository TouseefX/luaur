#[cfg(test)]
#[test]
fn compiler_compile_to_bytecode() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let source = String::from("return 5, 6.5");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\nLOADN R0 5\nLOADK R1 K0 [6.5]\nRETURN R0 2\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);

    let dump_all = bcb.dump_everything();
    let expected_all = "Function 0 (??):\nLOADN R0 5\nLOADK R1 K0 [6.5]\nRETURN R0 2\n\n";
    assert_eq!(dump_all, expected_all);
}
