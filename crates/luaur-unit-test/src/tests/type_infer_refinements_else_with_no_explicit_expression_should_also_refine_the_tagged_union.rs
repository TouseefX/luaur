//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1854:type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union

#[cfg(test)]
#[test]
fn type_infer_refinements_else_with_no_explicit_expression_should_also_refine_the_tagged_union() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Ok<T> = { tag: "ok", value: T }
        type Err<E> = { tag: "err", err: E }
        type Result<T, E> = Ok<T> | Err<E>

        function and_then<T, U, E>(r: Result<T, E>, f: (T) -> U): Result<U, E>
            if r.tag == "ok" then
                return { tag = "ok", value = f(r.value) }
            else
                return r
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
