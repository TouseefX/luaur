//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:4319:ir_lowering_custom_userdata_metamethod`
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
//!   - translates_to -> rust_item ir_lowering_custom_userdata_metamethod

#[cfg(test)]
#[test]
fn ir_lowering_custom_userdata_metamethod() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    if luau_codegen_supported() == 0 {
        return;
    }

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function foo(a: vec2, b: vec2, c: vec2)
    return -c + a * b
end
"#,
    )
    .unwrap();

    let actual = format!(
        "\n{}",
        fixture.get_codegen_assembly(source.as_ptr(), true, 1, 2, false)
    );
    let expected = r#"
; function foo($arg0, $arg1, $arg2) line 2
; R0: vec2 [argument]
; R1: vec2 [argument]
; R2: vec2 [argument]
bb_0:
  CHECK_TAG R0, tuserdata, exit(entry)
  CHECK_TAG R1, tuserdata, exit(entry)
  CHECK_TAG R2, tuserdata, exit(entry)
  JUMP bb_2
bb_2:
  JUMP bb_bytecode_1
bb_bytecode_1:
  %10 = LOAD_POINTER R2
  CHECK_USERDATA_TAG %10, 12i, exit(0)
  %12 = BUFFER_READF32 %10, 0i, tuserdata
  %13 = BUFFER_READF32 %10, 4i, tuserdata
  %14 = UNM_FLOAT %12
  %15 = UNM_FLOAT %13
  CHECK_GC
  %17 = NEW_USERDATA 8i, 12i
  BUFFER_WRITEF32 %17, 0i, %14, tuserdata
  BUFFER_WRITEF32 %17, 4i, %15, tuserdata
  %26 = LOAD_POINTER R0
  CHECK_USERDATA_TAG %26, 12i, bb_exit_3
   ; exit sync: R4, {%17}
  %28 = LOAD_POINTER R1
  CHECK_USERDATA_TAG %28, 12i, bb_exit_4
   ; exit sync: R4, {%17}
  %30 = BUFFER_READF32 %26, 0i, tuserdata
  %31 = BUFFER_READF32 %28, 0i, tuserdata
  %32 = MUL_FLOAT %30, %31
  %33 = BUFFER_READF32 %26, 4i, tuserdata
  %34 = BUFFER_READF32 %28, 4i, tuserdata
  %35 = MUL_FLOAT %33, %34
  %52 = ADD_FLOAT %14, %32
  %55 = ADD_FLOAT %15, %35
  %57 = NEW_USERDATA 8i, 12i
  BUFFER_WRITEF32 %57, 0i, %52, tuserdata
  BUFFER_WRITEF32 %57, 4i, %55, tuserdata
  STORE_POINTER R3, %57
  STORE_TAG R3, tuserdata
  INTERRUPT 3u
  RETURN R3, 1i
"#;

    assert_eq!(actual, expected);
}
