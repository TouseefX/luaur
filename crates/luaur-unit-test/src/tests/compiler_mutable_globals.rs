#[cfg(test)]
#[test]
fn compiler_mutable_globals() {
    use crate::functions::compile_function_0::compile_function_0;
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
    use luaur_compiler::records::compile_options::CompileOptions;

    let source = r#"
print()
Game.print()
Workspace.print()
_G.print()
game.print()
plugin.print()
script.print()
shared.print()
workspace.print()
"#;

    let result = compile_function_0(source);
    let expected = "\nGETIMPORT R0 1 [print]\nCALL R0 0 0\nGETIMPORT R0 3 [Game.print]\nCALL R0 0 0\nGETIMPORT R0 5 [Workspace.print]\nCALL R0 0 0\nGETIMPORT R0 7 [_G]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 9 [game.print]\nCALL R0 0 0\nGETIMPORT R0 11 [plugin.print]\nCALL R0 0 0\nGETIMPORT R0 13 [script.print]\nCALL R0 0 0\nGETIMPORT R0 15 [shared.print]\nCALL R0 0 0\nGETIMPORT R0 17 [workspace.print]\nCALL R0 0 0\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &result, expected);

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);
    let mut options = CompileOptions::default();
    let mutable_globals: [*const core::ffi::c_char; 8] = [
        c"Game".as_ptr(),
        c"Workspace".as_ptr(),
        c"game".as_ptr(),
        c"plugin".as_ptr(),
        c"script".as_ptr(),
        c"shared".as_ptr(),
        c"workspace".as_ptr(),
        core::ptr::null(),
    ];
    options.mutable_globals = mutable_globals.as_ptr();

    let source_str = String::from(source);
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source_str,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\nGETIMPORT R0 1 [print]\nCALL R0 0 0\nGETIMPORT R0 3 [Game]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 5 [Workspace]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 7 [_G]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 9 [game]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 11 [plugin]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 13 [script]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 15 [shared]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nGETIMPORT R0 17 [workspace]\nGETTABLEKS R0 R0 K0 ['print']\nCALL R0 0 0\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
