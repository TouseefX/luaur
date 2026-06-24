//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_of_an_imported_recursive_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
export type X = { a: number, b: X? }
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
type X = Import.X
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
    assert_eq!(unsafe { follow_type_id(ty1) }, unsafe {
        follow_type_id(ty2)
    });
}
