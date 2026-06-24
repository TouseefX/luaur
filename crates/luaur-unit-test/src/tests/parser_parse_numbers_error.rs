#[cfg(test)]
#[test]
fn parser_parse_numbers_error() {
    use crate::records::fixture::Fixture;
    use luaur_common::FFlag;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("return 0b123"),
        &alloc::string::String::from("Malformed number"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return 123x"),
        &alloc::string::String::from("Malformed number"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return 0xg"),
        &alloc::string::String::from("Malformed number"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return 0x0x123"),
        &alloc::string::String::from("Malformed number"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return 0xffffffffffffffffffffllllllg"),
        &alloc::string::String::from("Malformed number"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return 0x0xffffffffffffffffffffffffffff"),
        &alloc::string::String::from("Malformed number"),
        None,
    );
    if FFlag::LuauIntegerType2.get() {
        fixture.match_parse_error(
            &alloc::string::String::from("return 0x0xABCi"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 0xABCMi"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 0b250i"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 0bbbbi"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 123ii"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 0xABii"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 99999999999999999999i"),
            &alloc::string::String::from("Integer overflow"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 0xFFFFFFFFFFFFFFFFFFi"),
            &alloc::string::String::from("Integer overflow"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from(
                "return 0b10000000000000000000000000000000000000000000000000000000000000000i",
            ),
            &alloc::string::String::from("Integer overflow"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 123ii"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
        fixture.match_parse_error(
            &alloc::string::String::from("return 0xABii"),
            &alloc::string::String::from("Malformed integer"),
            None,
        );
    }
}
