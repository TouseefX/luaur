//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2140:type_infer_functions_other_things_are_not_related_to_function`
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
//!   - calls -> function registerHiddenTypes (tests/Fixture.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_functions_other_things_are_not_related_to_function

#[cfg(test)]
#[test]
fn type_infer_functions_other_things_are_not_related_to_function() {
    use crate::functions::register_hidden_types::register_hidden_types;
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    register_hidden_types(fixture.get_frontend());

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: fun = function() end
        local b: {} = a
        local c: boolean = a
        local d: fun = true
        local e: fun = {}
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    assert_eq!(2, result.errors[0].location.begin.line);
    assert_eq!(3, result.errors[1].location.begin.line);
    assert_eq!(4, result.errors[2].location.begin.line);
    assert_eq!(5, result.errors[3].location.begin.line);
}
