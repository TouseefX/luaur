//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:829:bytecode_call_inliner_retain_target_on_block_split`
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
//!   - translates_to -> rust_item bytecode_call_inliner_retain_target_on_block_split

#[cfg(test)]
#[test]
fn bytecode_call_inliner_retain_target_on_block_split() {
    use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_feedback = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
        alloc::format!(
            "\n{}",
            fixture.inline_and_print(
                r#"
        local function inlinee(a)
            return a + 1
        end

        local function caller(n)
            local sum = 0
            for i = 1, n do
                sum = sum + inlinee(i)
            end
            return sum
        end
    "#,
                0,
            )
        ),
        r#"
LOADK R1 K0 [0]
LOADK R4 K1 [1]
MOVE R2 R0
LOADN R3 1
FORNPREP R2 L3
L0: GETUPVAL R5 0
MOVE R6 R4
CMPPROTO R5 #0 L1
LOADK R8 K1 [1]
ADD R7 R6 R8
MOVE R5 R7
JUMP L2
L1: CALLFB R5 1 1 [0]
L2: ADD R1 R1 R5
FORNLOOP R2 L0
L3: RETURN R1 1
"#
    );
}
