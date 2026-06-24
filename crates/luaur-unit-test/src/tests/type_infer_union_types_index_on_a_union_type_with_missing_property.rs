//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_index_on_a_union_type_with_missing_property() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::missing_union_property::MissingUnionProperty;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = {x: number}
        type B = {}

        function f(t: A | B)
            return t.x
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mup = unsafe { get_type_error::<MissingUnionProperty>(&result.errors[0]).as_ref() };
    assert!(mup.is_some());
    assert_eq!(
        "Key 'x' is missing from 'B' in the type 'A | B'",
        to_string_type_error(&result.errors[0])
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "(A | B) -> number"
    } else {
        "(A | B) -> *error-type*"
    };

    assert_eq!(
        expected,
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
}
