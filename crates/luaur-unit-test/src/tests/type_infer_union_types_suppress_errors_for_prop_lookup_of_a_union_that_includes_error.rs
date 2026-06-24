//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_suppress_errors_for_prop_lookup_of_a_union_that_includes_error() {
    use crate::functions::register_hidden_types::register_hidden_types;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    register_hidden_types(fixture.get_frontend());

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(a: err | Not<nil>)
            local b = a.foo
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
