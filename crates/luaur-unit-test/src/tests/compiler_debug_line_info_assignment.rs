#[cfg(test)]
#[test]
fn compiler_debug_line_info_assignment() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_common::FFlag;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let _scoped_flag = ScopedFastFlag::new(&FFlag::LuauCompileDuptableConstantPack2, true);

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from(
        "\n   local a = { b = { c = { d = 3 } } }\n\na\n[\"b\"]\n[\"c\"]\n[\"d\"] = 4\n",
    );
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n2: DUPTABLE R0 1\n2: DUPTABLE R1 3\n2: DUPTABLE R2 6\n2: SETTABLEKS R2 R1 K2 ['c']\n2: SETTABLEKS R1 R0 K0 ['b']\n5: GETTABLEKS R2 R0 K0 ['b']\n6: GETTABLEKS R1 R2 K2 ['c']\n7: LOADN R2 4\n7: SETTABLEKS R2 R1 K4 ['d']\n8: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
