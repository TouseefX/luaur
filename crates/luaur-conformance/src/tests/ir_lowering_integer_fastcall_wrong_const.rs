//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/IrLowering.test.cpp:8137:ir_lowering_integer_fastcall_wrong_const`
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
//!   - calls -> method AssemblyBuilderX64::div (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::idiv (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderA64::udiv (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method AssemblyBuilderA64::rem (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function lrotate (CodeGen/src/BitUtils.h)
//!   - calls -> function rrotate (CodeGen/src/BitUtils.h)
//!   - translates_to -> rust_item ir_lowering_integer_fastcall_wrong_const

#[cfg(test)]
#[test]
fn ir_lowering_integer_fastcall_wrong_const() {
    use crate::records::lowering_fixture::LoweringFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _luau_integer_fastcalls = ScopedFastFlag::new(&FFlag::LuauIntegerFastcalls, true);
    let _luau_codegen_integer_2 = ScopedFastFlag::new(&FFlag::LuauCodegenInteger2, true);

    let mut fixture = LoweringFixture::default();
    let source = CString::new(
        r#"
local function f(...)
    integer.add(..., 0.5)
    integer.sub(..., 0.5)
    integer.mul(..., 0.5)
    integer.div(..., 0.5)
    integer.idiv(..., 0.5)
    integer.udiv(..., 0.5)
    integer.rem(..., 0.5)
    integer.urem(..., 0.5)
    integer.mod(..., 0.5)

    integer.min(..., 0.5)
    integer.max(..., 0.5)

    integer.band(..., 0.5)
    integer.bor(..., 0.5)
    integer.bxor(..., 0.5)
    integer.btest(..., 0.5)

    integer.extract(..., 0.5)

    integer.lrotate(..., 0.5)
    integer.rrotate(..., 0.5)
    integer.lshift(..., 0.5)
    integer.rshift(..., 0.5)
    integer.arshift(..., 0.5)

    integer.lt(..., 0.5)
    integer.le(..., 0.5)
    integer.gt(..., 0.5)
    integer.ge(..., 0.5)
    integer.ult(..., 0.5)
    integer.ule(..., 0.5)
    integer.ugt(..., 0.5)
    integer.uge(..., 0.5)
end
"#,
    )
    .unwrap();

    let assembly = fixture.get_codegen_assembly(source.as_ptr(), false, 1, 2, false);
    assert!(!assembly.is_empty());
}
