#[cfg(test)]
#[test]
fn compiler_basic_function_call() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let source = alloc::string::String::from(
        "local function foo(a, b) return b end function test() return foo(2) end",
    );
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(1);
    let expected_func = "\nGETUPVAL R0 0\nLOADN R1 2\nCALL R0 1 -1\nRETURN R0 -1\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
