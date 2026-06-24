//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1927:type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site

#[cfg(test)]
#[test]
fn type_infer_functions_dont_infer_parameter_types_for_functions_from_their_call_site() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {}

        function t.f(x)
            return x
        end

        t.__index = t

        function g(s)
            local q = s.p and s.p.q or nil
            return q and t.f(q) or nil
        end

        local f = t.f
    "#,
        ),
        None,
    );

    assert_eq!(
        "<a>(a) -> a",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "({+ p: {+ q: nil +} +}) -> nil",
            to_string_type_id(fixture.require_type_string(&String::from("g")))
        );
    }
}
