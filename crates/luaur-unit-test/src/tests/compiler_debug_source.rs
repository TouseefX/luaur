#[cfg(test)]
#[test]
fn compiler_debug_source() {
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    // Faithful port of the C++ R"(...)" source (leading + trailing newline).
    let source = String::from(
        "\nlocal kSelectedBiomes = {\n    ['Mountains'] = true,\n    ['Canyons'] = true,\n    ['Dunes'] = true,\n    ['Arctic'] = true,\n    ['Lavaflow'] = true,\n    ['Hills'] = true,\n    ['Plains'] = true,\n    ['Marsh'] = true,\n    ['Water'] = true,\n}\nlocal result = \"\"\nfor k in pairs(kSelectedBiomes) do\n    result = result .. k\nend\nreturn result\n",
    );

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_SOURCE);
    bcb.set_dump_source(&source);

    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(0);
    let expected_func = "\n    2: local kSelectedBiomes = {\nNEWTABLE R0 16 0\n    3:     ['Mountains'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K0 ['Mountains']\n    4:     ['Canyons'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K1 ['Canyons']\n    5:     ['Dunes'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K2 ['Dunes']\n    6:     ['Arctic'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K3 ['Arctic']\n    7:     ['Lavaflow'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K4 ['Lavaflow']\n    8:     ['Hills'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K5 ['Hills']\n    9:     ['Plains'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K6 ['Plains']\n   10:     ['Marsh'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K7 ['Marsh']\n   11:     ['Water'] = true,\nLOADB R1 1\nSETTABLEKS R1 R0 K8 ['Water']\n   13: local result = \"\"\nLOADK R1 K9 ['']\n   14: for k in pairs(kSelectedBiomes) do\nGETIMPORT R2 11 [pairs]\nMOVE R3 R0\nCALL R2 1 3\nFORGPREP_NEXT R2 L1\n   15:     result = result .. k\nL0: MOVE R7 R1\nMOVE R8 R5\nCONCAT R1 R7 R8\n   14: for k in pairs(kSelectedBiomes) do\nL1: FORGLOOP R2 L0 1\n   17: return result\nRETURN R1 1\n";

    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
