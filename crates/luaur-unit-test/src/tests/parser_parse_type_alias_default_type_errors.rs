#[cfg(test)]
#[test]
fn parser_parse_type_alias_default_type_errors() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("type Y<T = number, U> = {}"),
        &alloc::string::String::from("Expected default type after type name"),
        Some(Location::new(Position::new(0, 20), Position::new(0, 21))),
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type Y<T... = ...number, U...> = {}"),
        &alloc::string::String::from("Expected default type pack after type pack name"),
        Some(Location::new(Position::new(0, 29), Position::new(0, 30))),
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type Y<T... = (string) -> number> = {}"),
        &alloc::string::String::from("Expected type pack after '=', got type"),
        Some(Location::new(Position::new(0, 14), Position::new(0, 32))),
    );
}
