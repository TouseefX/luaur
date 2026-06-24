#[cfg(test)]
#[test]
fn compiler_builtin_folding_prohibited_in_options() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let mut options = luaur_compiler::records::compile_options::CompileOptions::default();
    options.optimization_level = 2;

    // math.floor from the test is excluded in this list on purpose
    let disabled_builtins: [*const core::ffi::c_char; 4] = [
        c"tostring".as_ptr(),
        c"math.abs".as_ptr(),
        c"math.sqrt".as_ptr(),
        core::ptr::null(),
    ];
    options.disabled_builtins = disabled_builtins.as_ptr();

    let source =
        String::from("return math.abs(-42), math.floor(-1.5), math.sqrt(9), (tostring(2))");
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let result = bcb.dump_function(0);
    let expected = "\nGETIMPORT R0 2 [math.abs]\nLOADN R1 -42\nCALL R0 1 1\nLOADN R1 -2\nGETIMPORT R2 4 [math.sqrt]\nLOADN R3 9\nCALL R2 1 1\nGETIMPORT R3 6 [tostring]\nLOADN R4 2\nCALL R3 1 1\nRETURN R0 4\n";

    assert_eq!("\n".to_string() + &result, expected);
}
