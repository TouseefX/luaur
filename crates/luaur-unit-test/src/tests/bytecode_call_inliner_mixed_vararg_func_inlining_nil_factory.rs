//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:591:bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory`
//! Source: `tests/BytecodeCallInliner.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/BytecodeCallInliner.test.cpp
//! - source_includes:
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
//!   - includes -> source_file Common/include/Luau/BytecodeWire.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeCallInliner.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/BytecodeCallInliner.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
//!   - translates_to -> rust_item bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory

#[cfg(test)]
#[test]
fn bytecode_call_inliner_mixed_vararg_func_inlining_nil_factory() {
    use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_feedback = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
        alloc::format!(
            "\n{}",
            fixture.inline_and_print(
                r#"
        local function inlinee(a, b, ...)
            local c, d = ...
            return a + b + c + d
        end
        local function caller(x)
            local result = inlinee(x, 100)
            return result + 2
        end
    "#,
                0,
            )
        ),
        r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [100]
CMPPROTO R1 #0 L0
LOADNIL R6
LOADNIL R7
ADD R10 R2 R3
ADD R9 R10 R6
ADD R8 R9 R7
MOVE R1 R8
JUMP L1
L0: CALLFB R1 2 1 [0]
L1: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
}
