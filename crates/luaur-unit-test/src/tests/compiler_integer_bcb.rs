#[cfg(test)]
#[test]
fn compiler_integer_bcb() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_common::FFlag::LuauIntegerType2;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
    use luaur_compiler::records::compile_options::CompileOptions;

    let _luau_integer = ScopedFastFlag::new(&LuauIntegerType2, true);

    let source = String::from("function foo()\nlocal a = 123i\nreturn a\nend");

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_TYPES);
    bcb.set_dump_source(&source);

    let mut options = CompileOptions::default();
    options.type_info_level = 1;
    options.optimization_level = 1;
    options.debug_level = 2;

    let parse_options = ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\nR0: integer from 0 to 2\nLOADK R0 K0 [123]\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
