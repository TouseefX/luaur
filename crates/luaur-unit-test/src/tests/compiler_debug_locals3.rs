#[cfg(test)]
#[test]
fn compiler_debug_locals3() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(
        BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES | BytecodeBuilder::DUMP_LOCALS,
    );
    bcb.set_dump_source("\nfunction foo(x)\n    repeat\n        local a, b\n        do continue end\n        local c, d = 2\n    until true\nend\n");

    let mut options = luaur_compiler::records::compile_options::CompileOptions::default();
    options.debug_level = 2;

    let source = String::from("\nfunction foo(x)\n    repeat\n        local a, b\n        do continue end\n        local c, d = 2\n    until true\nend\n");
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\nlocal 0: reg 3, start pc 5 line 8, no live range\nlocal 1: reg 4, start pc 5 line 8, no live range\nlocal 2: reg 1, start pc 2 line 5, end pc 4 line 6\nlocal 3: reg 2, start pc 2 line 5, end pc 4 line 6\nlocal 4: reg 0, start pc 0 line 4, end pc 5 line 8\n4: LOADNIL R1\n4: LOADNIL R2\n5: RETURN R0 0\n6: LOADN R3 2\n6: LOADNIL R4\n8: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
