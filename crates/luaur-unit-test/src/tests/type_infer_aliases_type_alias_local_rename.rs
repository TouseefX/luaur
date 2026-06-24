//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_local_rename() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Cool = { a: number, b: string }
type NotCool = Cool
local c: Cool = { a = 1, b = "s" }
local d: NotCool = { a = 1, b = "s" }
"#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "Cool",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
    assert_eq!(
        "NotCool",
        to_string_type_id(fixture.require_type_string(&String::from("d")))
    );
}
