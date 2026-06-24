//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6717:ir_lowering_fuzz_test_18`
//! Source: `tests/IrLowering.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrLowering.test.cpp
//! - source_includes:
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file tests/ConformanceIrHooks.h
//! - incoming:
//!   - declares <- source_file tests/IrLowering.test.cpp
//! - outgoing:
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_fuzz_test_18

#[cfg(test)]
#[test]
fn ir_lowering_fuzz_test_18() {
    use crate::records::lowering_fixture::LoweringFixture;
    use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    fixture.assembly_options.compilation_options.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;
    fixture.compilation_options.type_info_level = 0;

    let source = CString::new(
        r#"
_[_](_)
local _ = 538976256,_()()
do end
_ = 28672,false,_ ~= _ - _ - _ / _ >= _ - _ - _ / _ - _ - _ - "" - _ - _,not _ - "",not _ - _ - _,_
"#,
    )
    .unwrap();

    let actual = fixture.get_codegen_assembly(source.as_ptr(), false, 1, 1, false);

    assert!(!actual.is_empty());
}
