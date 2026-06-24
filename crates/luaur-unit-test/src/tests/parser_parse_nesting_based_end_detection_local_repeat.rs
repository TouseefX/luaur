#[cfg(test)]
#[test]
fn parser_parse_nesting_based_end_detection_local_repeat() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "-- i am line 1
repeat
  print(1)
  repeat
    print(2)
  print(3)
until false
        ",
    );

    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert_eq!(1, result.errors.len());

    let expected_message = "Expected 'until' (to close 'repeat' at line 2), got <eof>; did you forget to close 'repeat' at line 4?";
    assert_eq!(expected_message, result.errors[0].get_message().as_str());
}
