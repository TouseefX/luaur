#[cfg(test)]
#[test]
fn parser_parse_type_pack_errors() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("type Y<T...> = {a: T..., b: number}"),
        &alloc::string::String::from(
            "Unexpected '...' after type name; type pack is not allowed in this context",
        ),
        Some(Location::new(
            luaur_ast::records::position::Position::new(0, 20),
            luaur_ast::records::position::Position::new(0, 23),
        )),
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type Y<T...> = {a: (number | string)..."),
        &alloc::string::String::from("Unexpected '...' after type annotation"),
        Some(Location::new(
            luaur_ast::records::position::Position::new(0, 36),
            luaur_ast::records::position::Position::new(0, 39),
        )),
    );
}
