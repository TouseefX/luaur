//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/FeedbackVector.test.cpp:340:feedback_vector_namecall`
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
//!   - calls -> method BytecodeBuilder::dumpFunction (Bytecode/src/BytecodeBuilder.cpp)
//!   - calls -> method FeedbackVectorFixture::load (tests/FeedbackVector.test.cpp)
//!   - type_ref -> record FeedbackVectorSlot (VM/src/lobject.h)
//!   - type_ref -> enum FeedbackVectorSlotKind (VM/src/lobject.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record AssertInlinerData (tests/FeedbackVector.test.cpp)
//!   - calls -> function idInlinerWithAssert (tests/FeedbackVector.test.cpp)
//!   - calls -> method FeedbackVectorFixture::run (tests/FeedbackVector.test.cpp)
//!   - translates_to -> rust_item feedback_vector_namecall

#[cfg(test)]
#[test]
fn feedback_vector_namecall() {
    use crate::functions::id_inliner_with_assert::id_inliner_with_assert;
    use crate::records::assert_inliner_data::AssertInlinerData;
    use crate::records::feedback_vector_fixture::FeedbackVectorFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::enums::luau_proto_flag::LuauProtoFlag;
    use luaur_common::{FFlag, FInt};
    use luaur_vm::enums::feedback_vector_slot_kind::FeedbackVectorSlotKind;

    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
    let _call_fb = ScopedFastFlag::new(&FFlag::LuauCallFeedback, true);
    let _inline_threshold = ScopedFastInt::new(&FInt::LuauInlineHitsThreshold, 2);

    let mut fixture = FeedbackVectorFixture::new();
    fixture.compile(String::from(
        r#"
        local t = { x = 1 }
        function t.g(self) return self.x end
        local function f(t) return t:g() + 1 end
        f(t)
        f(t)
    "#,
    ));

    assert_eq!(
        format!("\n{}", fixture.bcb.dump_function(1)),
        r#"
NAMECALL R2 R0 K0 ['g']
CALLFB R2 1 1 [0]
LOADK R3 K1 [1]
ADD R1 R2 R3
RETURN R1 1
"#
    );

    let top = fixture.load();

    unsafe {
        let g = *(*top).p.add(0);
        assert_ne!((*g).flags & LuauProtoFlag::LPF_INLINABLE as u8, 0);

        let f = *(*top).p.add(1);
        assert_eq!((*f).feedbackvecsize, 1);

        let fbslot = (*f).feedbackvec.add(0);
        assert_eq!((*fbslot).kind, FeedbackVectorSlotKind::CALL_TARGET);
        assert_eq!((*fbslot).data.call_target.pc, 2);
        assert_eq!((*fbslot).data.call_target.proto, 0);
        assert_eq!((*fbslot).data.call_target.hits, 0);
        assert_eq!(
            *(*f).code.add((*fbslot).data.call_target.pc as usize + 1),
            0
        );

        let data = (*(*fixture.lua_state()).global).ecbdata.as_mut_ptr() as *mut AssertInlinerData;
        data.write(AssertInlinerData {
            proto: f,
            target: g,
            pc: (*fbslot).data.call_target.pc,
            called: false,
        });
        fixture.on_inline = Some(id_inliner_with_assert);

        fixture.run();

        assert_eq!((*fbslot).data.call_target.proto, (*g).funid);
        assert_eq!((*fbslot).data.call_target.hits, 2);
        assert_eq!((*data).called, true);
    }
}
