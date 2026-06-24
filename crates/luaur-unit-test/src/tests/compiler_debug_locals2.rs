#[cfg(test)]
#[test]
fn compiler_debug_locals2() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let source =
        String::from("\nfunction foo(x)\n    repeat\n        local a, b\n    until true\nend\n");

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(
        BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES | BytecodeBuilder::DUMP_LOCALS,
    );
    bcb.set_dump_source(&source);

    let options = luaur_compiler::records::compile_options::CompileOptions {
        debug_level: 2,
        ..Default::default()
    };

    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\nlocal 0: reg 1, start pc 2 line 6, no live range\nlocal 1: reg 2, start pc 2 line 6, no live range\nlocal 2: reg 0, start pc 0 line 4, end pc 2 line 6\n4: LOADNIL R1\n4: LOADNIL R2\n6: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
