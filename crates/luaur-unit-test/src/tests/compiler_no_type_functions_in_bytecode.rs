#[cfg(test)]
#[test]
fn compiler_no_type_functions_in_bytecode() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let source = String::from(
        "type function a() return types.any end\nfunction b() return 2 end\nreturn b()",
    );
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_all = bcb.dump_everything();
    let expected_all = "\nFunction 0 (b):\nLOADN R0 2\nRETURN R0 1\n\nFunction 1 (??):\nDUPCLOSURE R0 K0 ['b']\nSETGLOBAL R0 K1 ['b']\nGETGLOBAL R0 K1 ['b']\nCALL R0 0 -1\nRETURN R0 -1\n\n";
    assert_eq!("\n".to_string() + &dump_all, expected_all);
}
