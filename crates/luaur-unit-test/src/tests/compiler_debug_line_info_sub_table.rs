#[cfg(test)]
#[test]
fn compiler_debug_line_info_sub_table() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let _luau_compile_duptable_constant_pack2 =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileDuptableConstantPack2, true);

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from("\nlocal Value1, Value2, Value3 = ...\nlocal Table = {}\n\nTable.SubTable[\"Key\"] = {\n    Key1 = Value1,\n    Key2 = Value2,\n    Key3 = Value3,\n    Key4 = true,\n}\n");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n2: GETVARARGS R0 3\n3: NEWTABLE R3 0 0\n5: GETTABLEKS R4 R3 K0 ['SubTable']\n5: DUPTABLE R5 6\n6: SETTABLEKS R0 R5 K1 ['Key1']\n7: SETTABLEKS R1 R5 K2 ['Key2']\n8: SETTABLEKS R2 R5 K3 ['Key3']\n5: SETTABLEKS R5 R4 K7 ['Key']\n11: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
