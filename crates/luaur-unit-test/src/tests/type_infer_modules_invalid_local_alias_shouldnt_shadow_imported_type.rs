//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_invalid_local_alias_shouldnt_shadow_imported_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::recursive_restraint_violation::RecursiveRestraintViolation;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        export type bad<T> = {T}
        return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local a_mod = require(game.A)
        type bad<T> = {bad<{T}>}
        type fine<T> = a_mod.bad<T>
        local f: fine<number>
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<RecursiveRestraintViolation>(&result.errors[0]).is_some());

    let b = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    let f_type = fixture
        .base
        .require_type_module_ptr_string(&b, &String::from("f"));
    assert_eq!("fine<number>", to_string_type_id(f_type));
}
