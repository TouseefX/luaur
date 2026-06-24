//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:8186:ir_lowering_number_fastcall_wrong_const`
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
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> function lrotate (CodeGen/src/BitUtils.h)
//!   - calls -> function rrotate (CodeGen/src/BitUtils.h)
//!   - translates_to -> rust_item ir_lowering_number_fastcall_wrong_const

#[cfg(test)]
#[test]
fn ir_lowering_number_fastcall_wrong_const() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _luau_codegen_integer_2 = ScopedFastFlag::new(&FFlag::LuauCodegenInteger2, true);
    let _luau_codegen_integer_fastcall_2k =
        ScopedFastFlag::new(&FFlag::LuauCodegenIntegerFastcall2k, true);
    let _luau_integer_type = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function f(...)
    -- 2-arg math
    math.pow(..., 5i)
    math.fmod(..., 5i)
    math.atan2(..., 5i)
    math.ldexp(..., 5i)
    math.min(..., 5i)
    math.max(..., 5i)

    -- bit32 multiarg
    bit32.band(..., 5i)
    bit32.bor(..., 5i)
    bit32.bxor(..., 5i)
    bit32.btest(..., 5i)

    -- bit32 shift/rotate
    bit32.lshift(..., 5i)
    bit32.rshift(..., 5i)
    bit32.arshift(..., 5i)
    bit32.lrotate(..., 5i)
    bit32.rrotate(..., 5i)

    -- bit32 extract (2-arg)
    bit32.extract(..., 5i)

    -- vector constructor (2-arg)
    vector.create(..., 5i)

    -- buffer reads (offset is checked as double)
    buffer.readi8(..., 5i)
    buffer.readu8(..., 5i)
    buffer.readi16(..., 5i)
    buffer.readu16(..., 5i)
    buffer.readi32(..., 5i)
    buffer.readu32(..., 5i)
    buffer.readf32(..., 5i)
    buffer.readf64(..., 5i)
end
"#,
    )
    .unwrap();

    let assembly = fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false);
    assert!(!assembly.is_empty());
}
