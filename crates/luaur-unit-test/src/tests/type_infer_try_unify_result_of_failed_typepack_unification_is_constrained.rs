//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:205:type_infer_try_unify_result_of_failed_typepack_unification_is_constrained`
//! Source: `tests/TypeInfer.tryUnify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_try_unify_result_of_failed_typepack_unification_is_constrained

#[cfg(test)]
#[test]
fn type_infer_try_unify_result_of_failed_typepack_unification_is_constrained() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(arg: number) return arg end
        local a
        local b
        local c = f(a, b)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
}
