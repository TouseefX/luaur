#[cfg(test)]
#[test]
fn compiler_debug_line_info_while() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from("\nlocal count = 0\nwhile true do\n    count += 1\n    if count > 1 then\n        print(\"done!\")\n        break\n    end\nend\n");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n2: LOADN R0 0\n4: L0: ADDK R0 R0 K0 [1]\n5: LOADN R1 1\n5: JUMPIFNOTLT R1 R0 L1\n6: GETIMPORT R1 2 [print]\n6: LOADK R2 K3 ['done!']\n6: CALL R1 1 0\n7: RETURN R0 0\n3: L1: JUMPBACK L0\n10: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
