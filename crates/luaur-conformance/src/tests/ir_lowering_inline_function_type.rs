//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3484:ir_lowering_inline_function_type`
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
//!   - translates_to -> rust_item ir_lowering_inline_function_type

#[cfg(test)]
#[test]
fn ir_lowering_inline_function_type() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function inl(v: vector, s: number)
    return v.Y * s
end

local function getsum(x)
    return inl(x, 3) + inl(x, 5)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function inl($arg0, $arg1) line 2
; R0: vector [argument]
; R1: number [argument]
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %8 = LOAD_FLOAT R0, 4i
  %9 = FLOAT_TO_NUM %8
  %18 = MUL_NUM %9, R1
  STORE_DOUBLE R2, %18
  STORE_TAG R2, tnumber
  INTERRUPT 3u
  RETURN R2, 1i
; function getsum($arg0) line 6
; R0: vector from 0 to 3
; R0: vector from 3 to 6
bb_bytecode_0:
  CHECK_TAG R0, tvector, exit(0)
  %2 = LOAD_FLOAT R0, 4i
  %3 = FLOAT_TO_NUM %2
  %9 = MUL_NUM %3, 3
  %21 = MUL_NUM %3, 5
  %30 = ADD_NUM %9, %21
  STORE_DOUBLE R1, %30
  STORE_TAG R1, tnumber
  INTERRUPT 7u
  RETURN R1, 1i
"#;

    assert_eq!(actual, expected);
}
