//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:3312:ir_lowering_fastcall_type_infer_through_upvalue`
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
//!   - calls -> macro upvalue (VM/src/lobject.h)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_fastcall_type_infer_through_upvalue

#[cfg(test)]
#[test]
fn ir_lowering_fastcall_type_infer_through_upvalue() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local v = ...

local function getsum(x, c)
    v = vector(x, 2, 3)
    if c then
        return v.X + v.Y
    else
        return v.Z
    end
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function getsum($arg0, $arg1) line 4
; U0: vector
bb_bytecode_0:
  implicit CHECK_SAFE_ENV exit(0)
  CHECK_TAG R0, tnumber, bb_exit_4
   ; exit sync: R5, R4, {}
  %11 = LOAD_DOUBLE R0
  %14 = NUM_TO_FLOAT %11
  STORE_VECTOR R2, %14, 2, 3
  STORE_TAG R2, tvector
  %20 = LOAD_TVALUE R2, 0i, tvector
  SET_UPVALUE U0, %20, tvector
  JUMP_IF_FALSY R1, bb_bytecode_1, bb_3
bb_3:
  %23 = GET_UPVALUE U0
  STORE_TVALUE R3, %23
  CHECK_TAG R3, tvector, exit(11)
  %27 = EXTRACT_VEC %23, 0i
  %28 = FLOAT_TO_NUM %27
  STORE_TVALUE R4, %23
  %35 = EXTRACT_VEC %23, 1i
  %36 = FLOAT_TO_NUM %35
  %45 = ADD_NUM %28, %36
  STORE_DOUBLE R2, %45
  STORE_TAG R2, tnumber
  INTERRUPT 17u
  RETURN R2, 1i
bb_bytecode_1:
  %50 = GET_UPVALUE U0
  STORE_TVALUE R2, %50
  CHECK_TAG R2, tvector, exit(19)
  %54 = EXTRACT_VEC %50, 2i
  %55 = FLOAT_TO_NUM %54
  STORE_DOUBLE R2, %55
  STORE_TAG R2, tnumber
  INTERRUPT 21u
  RETURN R2, 1i
"#;

    assert_eq!(actual, expected);
}
