#[cfg(test)]
#[test]
fn compiler_capture_self() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_common::FFlag::LuauEmitCallFeedback;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let _emit_call_fb = ScopedFastFlag::new(&LuauEmitCallFeedback, true);

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

    let source = String::from("local MaterialsListClass = {}\n\nfunction MaterialsListClass:_MakeToolTip(guiElement, text)\n    local function updateTooltipPosition()\n        self._tweakingTooltipFrame = 5\n    end\n\n    updateTooltipPosition()\nend\n\nreturn MaterialsListClass");
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func_1 = bcb.dump_function(1);
    let expected_func_1 =
        "\nNEWCLOSURE R3 P0\nCAPTURE VAL R0\nMOVE R4 R3\nCALLFB R4 0 0 [0]\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func_1, expected_func_1);

    let dump_func_0 = bcb.dump_function(0);
    let expected_func_0 =
        "\nGETUPVAL R0 0\nLOADN R1 5\nSETTABLEKS R1 R0 K0 ['_tweakingTooltipFrame']\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func_0, expected_func_0);
}
