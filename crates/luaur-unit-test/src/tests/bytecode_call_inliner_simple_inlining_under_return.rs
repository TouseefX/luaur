//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCallInliner.test.cpp:245:bytecode_call_inliner_simple_inlining_under_return`
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
//!   - type_ref -> record BytecodeBuilder (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - calls -> method BytecodeInlinerFixture::inlineAndPrint (tests/BytecodeCallInliner.test.cpp)
//!   - translates_to -> rust_item bytecode_call_inliner_simple_inlining_under_return

#[cfg(test)]
#[test]
fn bytecode_call_inliner_simple_inlining_under_return() {
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
            return a
        end

        local function caller()
            local r1, r2 = inlinee(10)
            return r2
        end
    "#,
                0,
            )
        ),
        r#"
GETUPVAL R0 0
LOADK R1 K0 [10]
CMPPROTO R0 #0 L0
MOVE R0 R1
LOADNIL R1
RETURN R1 1
L0: CALLFB R0 1 2 [0]
RETURN R1 1
"#
    );
}
