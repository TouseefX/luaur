//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:6468:ir_lowering_fuzz_test_4`
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
//!   - translates_to -> rust_item ir_lowering_fuzz_test_4

#[cfg(test)]
#[test]
fn ir_lowering_fuzz_test_4() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local _ = math.exp,_(),_
local _ = math._,_(_(_),_(_ and _),_(_(_),_,_,_()),`{nil}`),_
for l41=_,_ do
end
l0 -= _(0,_(# _,_(_),_(_(_),_(_),_,_()),`{nil}`))
"#,
    )
    .unwrap();

    let actual = fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false);

    assert!(!actual.is_empty());
}
