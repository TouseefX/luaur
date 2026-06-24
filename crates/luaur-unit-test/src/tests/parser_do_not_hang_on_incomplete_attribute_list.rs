#[cfg(test)]
#[test]
fn parser_do_not_hang_on_incomplete_attribute_list() {
    use crate::functions::check_first_error_for_attributes::check_first_error_for_attributes;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();

    let code1 = r"(
@[]
function hello(x, y)
    return x + y
end)";
    let result1 = fix.try_parse(
        &code1.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    let expected_location1 = Location::new(Position::new(1, 0), Position::new(1, 3));
    let expected_message1 = "Attribute list cannot be empty";
    check_first_error_for_attributes(&result1.errors, 1, expected_location1, expected_message1);

    let code2 = r"@[";
    let result2 = fix.try_parse(
        &code2.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    let expected_location2 = Location::new(Position::new(0, 2), Position::new(0, 2));
    let expected_message2 = "Expected identifier when parsing attribute name, got <eof>";
    check_first_error_for_attributes(&result2.errors, 1, expected_location2, expected_message2);

    let code3 = r"@[
        function foo() end
    )";
    let result3 = fix.try_parse(
        &code3.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    let expected_location3 = Location::new(Position::new(1, 8), Position::new(1, 16));
    let expected_message3 = "Expected identifier when parsing attribute name, got 'function'";
    check_first_error_for_attributes(&result3.errors, 1, expected_location3, expected_message3);

    let code4 = r"@[deprecated
        local function foo() end
    )";
    let result4 = fix.try_parse(
        &code4.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    let expected_location4 = Location::new(Position::new(1, 8), Position::new(1, 13));
    let expected_message4 = "Expected ']' (to close '@[' at line 1), got 'local'";
    check_first_error_for_attributes(&result4.errors, 1, expected_location4, expected_message4);
}
