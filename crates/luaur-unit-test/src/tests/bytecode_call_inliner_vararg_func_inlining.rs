//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:479:bytecode_call_inliner_vararg_func_inlining`
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
//!   - translates_to -> rust_item bytecode_call_inliner_vararg_func_inlining

#[cfg(test)]
#[test]
fn bytecode_call_inliner_vararg_func_inlining() {
    use crate::records::bytecode_inliner_fixture::BytecodeInlinerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_feedback = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let mut fixture = BytecodeInlinerFixture::new();

    assert_eq!(
        alloc::format!(
            "\n{}",
            fixture.inline_and_print(
                r#"
        local function inlinee(...)
            local x = 12
            local a, b = ...
            if b < 0 then return a - b end
            return a + b
        end

        local function caller(x)
            local result = inlinee(x, 42)
            return result + 2
        end
    "#,
                0,
            )
        ),
        r#"
GETUPVAL R1 0
MOVE R2 R0
LOADK R3 K0 [42]
CMPPROTO R1 #0 L1
LOADK R4 K2 [12]
MOVE R5 R2
MOVE R6 R3
LOADK R7 K3 [0]
JUMPIFNOTLT R6 R7 L0
SUB R7 R5 R6
MOVE R1 R7
JUMP L2
L0: ADD R7 R5 R6
MOVE R1 R7
JUMP L2
L1: CALLFB R1 2 1 [0]
L2: LOADK R3 K1 [2]
ADD R2 R1 R3
RETURN R2 1
"#
    );
}
