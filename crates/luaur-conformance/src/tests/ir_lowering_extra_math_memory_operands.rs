//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:611:ir_lowering_extra_math_memory_operands`
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
//!   - translates_to -> rust_item ir_lowering_extra_math_memory_operands

#[cfg(test)]
#[test]
fn ir_lowering_extra_math_memory_operands() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: number, b: number, c: number, d: number, e: number)
    return math.floor(a) + math.ceil(b) + math.round(c) + math.sqrt(d) + math.abs(e)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2, $arg3, $arg4) line 2
bb_0:
  CHECK_TAG R0, tnumber, exit(entry)
  CHECK_TAG R1, tnumber, exit(entry)
  CHECK_TAG R2, tnumber, exit(entry)
  CHECK_TAG R3, tnumber, exit(entry)
  CHECK_TAG R4, tnumber, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %16 = FLOOR_NUM R0
  %24 = CEIL_NUM R1
  %34 = ADD_NUM %16, %24
  %41 = ROUND_NUM R2
  %51 = ADD_NUM %34, %41
  %58 = SQRT_NUM R3
  %68 = ADD_NUM %51, %58
  %75 = ABS_NUM R4
  %85 = ADD_NUM %68, %75
  STORE_DOUBLE R5, %85
  STORE_TAG R5, tnumber
  INTERRUPT 29u
  RETURN R5, 1i
"#;

    assert_eq!(actual, expected);
}
