//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4162:ir_lowering_custom_userdata_namecall_2`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
//!   - calls -> method LoweringFixture::getCodegenAssembly (tests/IrLowering.test.cpp)
//!   - translates_to -> rust_item ir_lowering_custom_userdata_namecall_2

#[cfg(test)]
#[test]
fn ir_lowering_custom_userdata_namecall_2() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _load_propagate_origin = ScopedFastFlag::new(&FFlag::LuauCodegenLoadPropagateOrigin, true);

    if luau_codegen_supported() == 0 {
        return;
    }

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: vec2, b: vec2)
    return a:Min(b)
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1) line 2
; R0: vec2 [argument]
; R1: vec2 [argument]
bb_0:
  CHECK_TAG R0, tuserdata, exit(entry)
  CHECK_TAG R1, tuserdata, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %6 = LOAD_TVALUE R1, 0i, tuserdata
  STORE_TVALUE R4, %6
  %10 = LOAD_POINTER R0
  CHECK_USERDATA_TAG %10, 12i, exit(1)
  %14 = LOAD_POINTER R1
  CHECK_USERDATA_TAG %14, 12i, exit(1)
  %16 = BUFFER_READF32 %10, 0i, tuserdata
  %17 = BUFFER_READF32 %14, 0i, tuserdata
  %18 = FLOAT_TO_NUM %16
  %19 = FLOAT_TO_NUM %17
  %20 = MIN_NUM %18, %19
  %21 = BUFFER_READF32 %10, 4i, tuserdata
  %22 = BUFFER_READF32 %14, 4i, tuserdata
  %23 = FLOAT_TO_NUM %21
  %24 = FLOAT_TO_NUM %22
  %25 = MIN_NUM %23, %24
  %26 = NUM_TO_FLOAT %20
  %27 = NUM_TO_FLOAT %25
  CHECK_GC
  %29 = NEW_USERDATA 8i, 12i
  BUFFER_WRITEF32 %29, 0i, %26, tuserdata
  BUFFER_WRITEF32 %29, 4i, %27, tuserdata
  STORE_POINTER R2, %29
  STORE_TAG R2, tuserdata
  ADJUST_STACK_TO_REG R2, 1i
  INTERRUPT 4u
  RETURN R2, -1i
"#;

    assert_eq!(actual, expected);
}
