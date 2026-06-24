//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2846:type_infer_refinements_table_name_index_without_prior_assignment_from_branch`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_refinements_table_name_index_without_prior_assignment_from_branch

#[cfg(test)]
#[test]
fn type_infer_refinements_table_name_index_without_prior_assignment_from_branch() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::optional_value_access::OptionalValueAccess;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local GetDictionary : (unknown, boolean) -> { Player: {} }? = nil :: any

        local CharEntry = GetDictionary(nil, false)
        if not CharEntry then
            CharEntry = GetDictionary(nil, true)
        end

        local x = CharEntry.Player
    "#,
        ),
        None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    type_error_data_ref::<OptionalValueAccess>(&results.errors[0])
        .expect("expected OptionalValueAccess");
    assert_eq!(
        "{  }",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
}
