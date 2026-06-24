//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_generic_type_packs_in_non_strict() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = fixture.check_non_strict(&String::from(
        r#"
--!nonstrict
local test: <T...>(T...) -> () -- TypeError: Unknown type 'T'
"#,
    ));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
