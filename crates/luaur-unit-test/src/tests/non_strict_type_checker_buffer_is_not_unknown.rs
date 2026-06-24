//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_buffer_is_not_unknown() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_ast::enums::mode::Mode;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_mode_string_optional_frontend_options(
        Mode::Nonstrict,
        &String::from(
            r#"
local function wrap(b: buffer, i: number, v: number)
    buffer.writeu32(b, i * 4, v)
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
