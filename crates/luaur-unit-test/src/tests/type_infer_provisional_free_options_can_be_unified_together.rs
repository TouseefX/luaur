//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:938:type_infer_provisional_free_options_can_be_unified_together`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
//!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
//!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Unifier (Analysis/include/Luau/Unifier.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> enum Variance (Analysis/include/Luau/Unifier.h)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - translates_to -> rust_item type_infer_provisional_free_options_can_be_unified_together

#[cfg(test)]
#[test]
fn type_infer_provisional_free_options_can_be_unified_together() {
    use crate::records::try_unify_fixture::TryUnifyFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = TryUnifyFixture::new();
    let nil_type = fixture.get_builtins().nilType;

    let free1 = fixture.fresh_type();
    let option1 = fixture.arena.add_type(UnionType {
        options: vec![nil_type, free1],
    });

    let free2 = fixture.fresh_type();
    let option2 = fixture.arena.add_type(UnionType {
        options: vec![nil_type, free2],
    });

    fixture
        .state
        .try_unify_type_id_type_id_bool_bool_literal_properties(
            option1, option2, false, false, None,
        );
    assert!(!fixture.state.failure, "{:?}", fixture.state.errors);

    fixture.state.log.commit();

    // C++ declares a single `ToStringOptions opts;` and reuses it across both
    // toString calls, so the shared nameMap mints 'a for option1 and 'b for option2.
    let mut opts = ToStringOptions::default();
    assert_eq!(
        "'a?",
        to_string_type_id_to_string_options(option1, &mut opts)
    );
    assert_eq!(
        "'b?",
        to_string_type_id_to_string_options(option2, &mut opts)
    );
}
