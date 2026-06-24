//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:7649:ir_lowering_libm_is_pure`
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
//!   - translates_to -> rust_item ir_lowering_libm_is_pure

#[cfg(test)]
#[test]
fn ir_lowering_libm_is_pure() {
    use crate::records::lowering_fixture::LoweringFixture;
    use std::ffi::CString;

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(p: vector, v: vector): vector
    return vector.create(
        math.cos(0.6 * p.x + 0.4 * math.sin(v.y) + 0),
        math.cos(0.6 * p.x + 0.4 * math.sin(v.y) + 1),
        math.cos(0.6 * p.x + 0.4 * math.sin(v.y) + 2)
    )
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 2
bb_0:
  CHECK_TAG R0, tvector, exit(entry)
  CHECK_TAG R1, tvector, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  implicit CHECK_SAFE_ENV exit(0)
  %8 = LOAD_FLOAT R0, 0i
  %9 = FLOAT_TO_NUM %8
  %15 = MUL_NUM %9, 0.59999999999999998
  %20 = LOAD_FLOAT R1, 4i
  %21 = FLOAT_TO_NUM %20
  %28 = INVOKE_LIBM 24u, %21
  %35 = MUL_NUM %28, 0.40000000000000002
  %44 = ADD_NUM %15, %35
  %50 = ADD_NUM %44, 0
  %57 = INVOKE_LIBM 9u, %50
  %105 = ADD_NUM %44, 1
  %112 = INVOKE_LIBM 9u, %105
  %160 = ADD_NUM %44, 2
  %167 = INVOKE_LIBM 9u, %160
  %181 = NUM_TO_FLOAT %57
  %182 = NUM_TO_FLOAT %112
  %183 = NUM_TO_FLOAT %167
  STORE_VECTOR R2, %181, %182, %183
  STORE_TAG R2, tvector
  INTERRUPT 52u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
