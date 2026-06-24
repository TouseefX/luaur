//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_nonstrict_shouldnt_warn_on_valid_buffer_use() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;

    let mut fixture = NonStrictTypeCheckerFixture {
        base: crate::records::fixture::Fixture::fixture_bool(false),
        definitions: String::new(),
    };

    fixture.base.load_definition(
        &String::from(
            r#"
declare buffer: {
    create: @checked (size: number) -> buffer,
    readi8: @checked (b: buffer, offset: number) -> number,
    writef64: @checked (b: buffer, offset: number, value: number) -> (),
}
"#,
        ),
        false,
    );

    let result = fixture.check_non_strict(&String::from(
        r#"
local b = buffer.create(100)
buffer.writef64(b, 0, 5)
buffer.readi8(b, 0)
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
