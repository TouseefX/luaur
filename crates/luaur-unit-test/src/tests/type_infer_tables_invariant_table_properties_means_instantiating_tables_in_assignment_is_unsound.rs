//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2324:type_infer_tables_invariant_table_properties_means_instantiating_tables_in_assignment_is_unsound`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record ExplicitFunctionAnnotationRecommended (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_invariant_table_properties_means_instantiating_tables_in_assignment_is_unsound

#[cfg(test)]
#[test]
fn type_infer_tables_invariant_table_properties_means_instantiating_tables_in_assignment_is_unsound(
) {
    use crate::functions::has_error::has_error;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(
        &FFlag::LuauInstantiateInSubtyping,
        !FFlag::DebugLuauForceOldSolver.get(),
    );

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local t = {}
        function t.m(x) return x end
        local a : string = t.m("hi")
        local b : number = t.m(5)
        local u : { m : (number)->number } = t -- This shouldn't typecheck
        u.m = function(x) return 1+x end
        local c : string = t.m("hi")
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        assert!(
            has_error::<ExplicitFunctionAnnotationRecommended>(&result),
            "{:?}",
            result.errors
        );
        assert!(has_error::<TypeMismatch>(&result), "{:?}", result.errors);
    } else {
        assert!(!result.errors.is_empty(), "{:?}", result.errors);
    }
}
