//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_primitive() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from("local a = nil    local b = 44    local c = 'lalala'    local d = true"),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "nil",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
    } else {
        assert_ne!(
            "nil",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
    }

    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.require_type_string(&String::from("d")))
    );
}
