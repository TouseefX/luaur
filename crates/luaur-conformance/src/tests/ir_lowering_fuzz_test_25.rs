//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6868:ir_lowering_fuzz_test_25`
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
//!   - translates_to -> rust_item ir_lowering_fuzz_test_25

#[cfg(test)]
#[test]
fn ir_lowering_fuzz_test_25() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _record_all_block_exit_info =
        ScopedFastFlag::new(&FFlag::LuauCodegenRecordAllBlockExitInfo, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local _ = ...
local _ = function(l0,l4,l0: ()->())
    local _ = l0,_.n249 + l0
    n0,_,l0 = _,_,{},n0,_
    n0 *= _
    while true do
    end
end
_()
"#,
    )
    .unwrap();

    let actual = fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false);

    assert!(!actual.is_empty());
}
