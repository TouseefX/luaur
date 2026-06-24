#[cfg(test)]
#[test]
fn parser_can_parse_complex_unions_successfully() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FInt;

    let mut fixture = Fixture::fixture_bool(false);

    // Set recursion limit and type length limit to 10 for all tests in this fixture
    let _sfis = [
        ScopedFastInt::new(&FInt::LuauRecursionLimit, 10),
        ScopedFastInt::new(&FInt::LuauTypeLengthLimit, 10),
    ];

    // Test 1: Complex union with multiple function types, table types, parenthesized types, and intersection types
    fixture.parse(
        r#"local f:
() -> ()
|
() -> ()
|
{a: number}
|
{b: number}
|
((number))
|
((number))
|
(a & (b & nil))
|
(a & (b & nil))
"#,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    // Test 2: Union of nullable types
    fixture.parse(
        r#"local f: a? | b? | c? | d? | e? | f? | g? | h?
"#,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    // Test 3: Exceeded type length limit
    fixture.match_parse_error(
        &"local t: a & b & c & d & e & f & g & h & i & j & nil".to_string(),
        &"Exceeded allowed type length; simplify your type annotation to make the code compile"
            .to_string(),
        None,
    );
}
