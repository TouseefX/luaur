//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6739:ir_lowering_fuzz_test_19`
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
//!   - calls -> macro tonumber (VM/src/lvm.h)
//!   - translates_to -> rust_item ir_lowering_fuzz_test_19

#[cfg(test)]
#[test]
fn ir_lowering_fuzz_test_19() {
    use crate::records::lowering_fixture::LoweringFixture;
    use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    fixture.assembly_options.compilation_options.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;
    fixture.compilation_options.type_info_level = 0;

    let source = CString::new(
        r#"
local _ = tonumber(159)
_ += _
while 128 do
_,_ = vector.create(_,2304)
do end
while {_,[rawequal(_,_)]=l115,} do
end
end
while {1048576,[rawequal(_,_,_,_ + 128)]=l255,} do
_ = -5
end
_ += _
l0,_ = false,_
do end
"#,
    )
    .unwrap();

    let actual = fixture.get_codegen_assembly(source.as_ptr(), false, 1, 1, false);

    assert!(!actual.is_empty());
}
