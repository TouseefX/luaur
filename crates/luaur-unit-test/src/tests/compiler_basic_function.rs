#[cfg(test)]
#[test]
fn compiler_basic_function() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let source = String::from("local function foo(a, b) return b end");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(1);
    let expected_func = "\nDUPCLOSURE R0 K0 ['foo']\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);

    let dump_func0 = bcb.dump_function(0);
    let expected_func0 = "\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &dump_func0, expected_func0);
}
