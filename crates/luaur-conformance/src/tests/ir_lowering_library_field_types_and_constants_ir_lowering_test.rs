//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4432:ir_lowering_library_field_types_and_constants`
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
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item ir_lowering_library_field_types_and_constants

#[cfg(test)]
#[test]
fn ir_lowering_library_field_types_and_constants() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: vector)
    return Vector3.xAxis * a + Vector3.yAxis
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0) line 2
; R0: vector [argument]
; R2: vector from 3 to 4
; R3: vector from 1 to 2
; R3: vector from 3 to 4
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %4 = LOAD_TVALUE K0 (1, 0, 0), 0i, tvector
  %11 = LOAD_TVALUE R0, 0i, tvector
  %12 = MUL_VEC %4, %11
  %15 = LOAD_TVALUE K1 (0, 1, 0), 0i, tvector
  %23 = ADD_VEC %12, %15
  %24 = TAG_VECTOR %23
  STORE_TVALUE R1, %24
  INTERRUPT 4u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
