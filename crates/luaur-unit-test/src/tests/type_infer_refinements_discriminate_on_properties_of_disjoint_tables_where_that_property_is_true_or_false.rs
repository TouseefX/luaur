//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1391:type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false`
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
//!   - translates_to -> rust_item type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false

#[cfg(test)]
#[test]
fn type_infer_refinements_discriminate_on_properties_of_disjoint_tables_where_that_property_is_true_or_false(
) {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Ok<T> = { ok: true, value: T }
        type Err<E> = { ok: false, error: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function apply<T, E>(t: Result<T, E>, f: (T) -> (), g: (E) -> ())
            if t.ok then
                f(t.value)
            else
                g(t.error)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
