//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_report_shadowed_aliases() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::primitive_type::PrimitiveType;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type MyString = string
        type string = number
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Redefinition of type 'string'",
        to_string_type_error(&result.errors[0])
    );

    let t1 = fixture
        .lookup_type(&String::from("MyString"))
        .expect("expected MyString");
    assert_eq!(Some(PrimitiveType::String), fixture.get_primitive_type(t1));

    let t2 = fixture
        .lookup_type(&String::from("string"))
        .expect("expected string");
    assert_eq!(Some(PrimitiveType::String), fixture.get_primitive_type(t2));
}
