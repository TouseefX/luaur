//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_internal_type_errors_are_only_reported_once() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::internal_error::InternalError;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _magic_types = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
return function(): { X: _luau_blocked_type, Y: _luau_blocked_type } return nil :: any end
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<InternalError>(&result.errors[0]).is_some());

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    assert_eq!(
        "(...any) -> { X: *error-type*, Y: *error-type* }",
        to_string_type_pack_id(module.return_type)
    );
}
