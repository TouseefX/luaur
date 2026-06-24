//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/FeedbackVector.test.cpp:308:feedback_vector_metamethod_call_sealed`
//! Source: `tests/FeedbackVector.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/FeedbackVector.test.cpp
//! - source_includes:
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file VM/src/lstate.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/FeedbackVector.test.cpp
//! - outgoing:
//!   - calls -> method FeedbackVectorFixture::compile (tests/FeedbackVector.test.cpp)
//!   - calls -> method FeedbackVectorFixture::load (tests/FeedbackVector.test.cpp)
//!   - type_ref -> record FeedbackVectorSlot (VM/src/lobject.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> function idInliner (tests/FeedbackVector.test.cpp)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - calls -> function luaopen_base (VM/src/lbaselib.cpp)
//!   - calls -> method FeedbackVectorFixture::run (tests/FeedbackVector.test.cpp)
//!   - translates_to -> rust_item feedback_vector_metamethod_call_sealed

#[cfg(test)]
#[test]
fn feedback_vector_metamethod_call_sealed() {
    use crate::functions::id_inliner::id_inliner;
    use crate::records::feedback_vector_fixture::FeedbackVectorFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::macros::luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED;
    use luaur_common::{FFlag, FInt};
    use luaur_vm::functions::luaopen_base::luaopen_base;
    use luaur_vm::macros::lua_pop::lua_pop;

    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

    let mut fixture = FeedbackVectorFixture::new();
    fixture.compile(String::from(
        r#"
        local function f(h) return h(1) + 1 end

        local callableTable = {}

        setmetatable(callableTable, { __call = function(self, arg) return arg + 42 end })

        f(callableTable)
    "#,
    ));

    let top = fixture.load();

    unsafe {
        let f = *(*top).p.add(0);
        let fbslot = (*f).feedbackvec.add(0);

        assert_eq!(
            *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
            0
        );

        fixture.on_inline = Some(id_inliner);

        let l = fixture.lua_state();
        lua_pop(l, luaopen_base(l));

        fixture.run();

        assert_eq!(
            *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
            LUAU_INSN_FBSLOT_SEALED
        );
    }
}
