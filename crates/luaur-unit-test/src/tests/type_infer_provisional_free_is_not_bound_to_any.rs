//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:474:type_infer_provisional_free_is_not_bound_to_any`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_provisional_free_is_not_bound_to_any

#[cfg(test)]
#[test]
fn type_infer_provisional_free_is_not_bound_to_any() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function foo(f: (any) -> (), x)
            f(x)
        end
    "#,
        ),
        None,
    );

    assert_eq!(
        "((any) -> (), any) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
