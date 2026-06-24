//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_of_an_imported_recursive_generic_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        export type X<T, U> = { a: T, b: U, C: X<T, U>? }
        return {}
    "#,
        ),
    );

    let a_result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, a_result.errors.len(), "{:?}", a_result.errors);

    let b_result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Import = require(game.A)
        type X<T, U> = Import.X<T, U>
    "#,
        ),
        None,
    );
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let ty1 = fixture
        .base
        .lookup_imported_type(&String::from("Import"), &String::from("X"))
        .expect("expected imported type Import.X");
    let ty2 = fixture
        .base
        .lookup_type(&String::from("X"))
        .expect("expected local type X");
    let mut opts = ToStringOptions::to_string_options(true);
    let ty1_string = to_string_type_id_to_string_options(ty1, &mut opts);
    let mut opts = ToStringOptions::to_string_options(true);
    let ty2_string = to_string_type_id_to_string_options(ty2, &mut opts);
    assert_eq!(ty1_string, ty2_string);

    let b_result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Import = require(game.A)
        type X<T, U> = Import.X<U, T>
    "#,
        ),
        None,
    );
    assert_eq!(0, b_result.errors.len(), "{:?}", b_result.errors);

    let ty1 = fixture
        .base
        .lookup_imported_type(&String::from("Import"), &String::from("X"))
        .expect("expected imported type Import.X");
    let ty2 = fixture
        .base
        .lookup_type(&String::from("X"))
        .expect("expected local type X");

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "t1 where t1 = { C: t1?, a: T, b: U }",
        to_string_type_id_to_string_options(ty1, &mut opts)
    );

    let expected_ty2 = if !FFlag::DebugLuauForceOldSolver.get() {
        "t1 where t1 = { C: t1?, a: U, b: T }"
    } else {
        "{ C: t1, a: U, b: T } where t1 = { C: t1, a: U, b: T }?"
    };
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        expected_ty2,
        to_string_type_id_to_string_options(ty2, &mut opts)
    );
}
