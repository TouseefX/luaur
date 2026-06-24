//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_evaluating_generic_default_type_pack_shouldnt_ice() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::unknown_symbol::UnknownSymbol;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local A = {}
type B<T... = ...typeof(A)> = unknown
"#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    }
}
