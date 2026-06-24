//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:284:bytecode_call_inliner_namecall_inlining`
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
//!   - translates_to -> rust_item bytecode_call_inliner_namecall_inlining

#[cfg(test)]
#[test]
fn bytecode_call_inliner_namecall_inlining() {
    use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_feedback = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
        alloc::format!(
            "\n{}",
            fixture.inline_and_print(
                r#"
        local function inlinee(t, x)
            return t.v + x
        end

        local function caller(x)
            local t = {v = 7, inlinee = inlinee}
            local result = t:inlinee(42)
            return result + 2
        end
    "#,
                0,
            )
        ),
        r#"
DUPTABLE R1 2
LOADK R2 K0 ['v']
LOADK R3 K3 [7]
SETTABLE R3 R1 R2
LOADK R2 K1 ['inlinee']
GETUPVAL R3 0
SETTABLE R3 R1 R2
LOADK R4 K4 [42]
MOVE R3 R1
GETTABLEKS R2 R3 K1 ['inlinee']
CMPPROTO R2 #0 L0
GETTABLEKS R6 R3 K0 ['v']
ADD R5 R6 R4
MOVE R2 R5
JUMP L1
L0: NAMECALL R2 R1 K1 ['inlinee']
CALLFB R2 2 1 [0]
L1: LOADK R4 K5 [2]
ADD R3 R2 R4
RETURN R3 1
"#
    );
}
