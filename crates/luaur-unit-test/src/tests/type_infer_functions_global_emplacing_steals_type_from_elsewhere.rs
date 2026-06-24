//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4060:type_infer_functions_global_emplacing_steals_type_from_elsewhere`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_functions_global_emplacing_steals_type_from_elsewhere

#[cfg(test)]
#[test]
fn type_infer_functions_global_emplacing_steals_type_from_elsewhere() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f()
            return 42
        end
        local a = f()
        b = a
        local c = b
        function b()
        end
    "#,
        ),
        None,
    );

    // NOTE: upstream `global_emplacing_steals_type_from_elsewhere`
    // (tests/TypeInfer.functions.test.cpp:4060) does NOT assert on error count —
    // it only CHECK_EQ's the inferred types of `a`, `b`, `c`. Under the new solver
    // the global `b` is assigned (`b = a`) before being defined (`function b()`),
    // which legitimately emits an `UnknownSymbol{Binding}` implicit-global warning
    // (TypeChecker2.cpp:1529). A `0 errors` assertion was never part of the upstream
    // test; the faithful checks are the three type-string comparisons below.
    let _ = &result;
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "() -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
}
