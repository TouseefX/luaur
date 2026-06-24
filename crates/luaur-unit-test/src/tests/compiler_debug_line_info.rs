#[cfg(test)]
#[test]
fn compiler_debug_line_info() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);

    let source = String::from(
        r#"
local kSelectedBiomes = {
    ['Mountains'] = true,
    ['Canyons'] = true,
    ['Dunes'] = true,
    ['Arctic'] = true,
    ['Lavaflow'] = true,
    ['Hills'] = true,
    ['Plains'] = true,
    ['Marsh'] = true,
    ['Water'] = true,
}
local result = ""
for k in pairs(kSelectedBiomes) do
    result = result .. k
end
return result
"#,
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
    let expected_func = "\n2: NEWTABLE R0 16 0\n3: LOADB R1 1\n3: SETTABLEKS R1 R0 K0 ['Mountains']\n4: LOADB R1 1\n4: SETTABLEKS R1 R0 K1 ['Canyons']\n5: LOADB R1 1\n5: SETTABLEKS R1 R0 K2 ['Dunes']\n6: LOADB R1 1\n6: SETTABLEKS R1 R0 K3 ['Arctic']\n7: LOADB R1 1\n7: SETTABLEKS R1 R0 K4 ['Lavaflow']\n8: LOADB R1 1\n8: SETTABLEKS R1 R0 K5 ['Hills']\n9: LOADB R1 1\n9: SETTABLEKS R1 R0 K6 ['Plains']\n10: LOADB R1 1\n10: SETTABLEKS R1 R0 K7 ['Marsh']\n11: LOADB R1 1\n11: SETTABLEKS R1 R0 K8 ['Water']\n13: LOADK R1 K9 ['']\n14: GETIMPORT R2 11 [pairs]\n14: MOVE R3 R0\n14: CALL R2 1 3\n14: FORGPREP_NEXT R2 L1\n15: L0: MOVE R7 R1\n15: MOVE R8 R5\n15: CONCAT R1 R7 R8\n14: L1: FORGLOOP R2 L0 1\n17: RETURN R1 1\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
