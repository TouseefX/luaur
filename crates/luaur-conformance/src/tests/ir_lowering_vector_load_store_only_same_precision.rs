//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:5038:ir_lowering_vector_load_store_only_same_precision`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> type_alias vec (Common/include/Luau/InsertionOrderedMap.h)
//!   - translates_to -> rust_item ir_lowering_vector_load_store_only_same_precision

#[cfg(test)]
#[test]
fn ir_lowering_vector_load_store_only_same_precision() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function test(x: number, y: number)
    local vec = vector.create(x, y, 0)
    return vec.X + vec.Y + vec.Z
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function test($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %15 = LOAD_DOUBLE R0
  %16 = LOAD_DOUBLE R1
  %18 = NUM_TO_FLOAT %15
  %19 = NUM_TO_FLOAT %16
  %27 = FLOAT_TO_NUM %18
  %33 = FLOAT_TO_NUM %19
  %42 = ADD_NUM %27, %33
  %57 = ADD_NUM %42, 0
  STORE_DOUBLE R3, %57
  STORE_TAG R3, tnumber
  INTERRUPT 16u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
