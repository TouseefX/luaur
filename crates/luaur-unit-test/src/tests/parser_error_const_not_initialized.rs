#[cfg(test)]
#[test]
fn parser_error_const_not_initialized() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::LuauConst2, true);

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("const c"),
        &alloc::string::String::from("Missing initializer in const declaration"),
        None,
    );

    fixture.match_parse_error(
        &alloc::string::String::from("const a, b = nil"),
        &alloc::string::String::from("Missing initializer in const declaration"),
        None,
    );

    fixture.match_parse_error(
        &alloc::string::String::from("const a, b, c = f(), 42"),
        &alloc::string::String::from("Missing initializer in const declaration"),
        None,
    );

    fixture.match_parse_error(
        &alloc::string::String::from("const a, b, c = ..., 42"),
        &alloc::string::String::from("Missing initializer in const declaration"),
        None,
    );
}
