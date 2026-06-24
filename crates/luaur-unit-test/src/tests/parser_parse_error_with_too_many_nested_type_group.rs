#[cfg(test)]
#[test]
fn parser_parse_error_with_too_many_nested_type_group() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FInt;

    let mut fixture = Fixture::fixture_bool(false);

    // Set recursion limit to 10 for all tests in this fixture
    let _sfis = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    // Test 1: Too many nested parentheses in function return type
    fixture.match_parse_error(
        &"function f(): ((((((((((Fail)))))))))) end".to_string(),
        &"Exceeded allowed recursion depth; simplify your type annotation to make the code compile"
            .to_string(),
        None,
    );

    // Test 2: Too many nested function arrow types
    fixture.match_parse_error(
        &"function f(): () -> () -> () -> () -> () -> () -> () -> () -> () -> () -> () end"
            .to_string(),
        &"Exceeded allowed recursion depth; simplify your type annotation to make the code compile"
            .to_string(),
        None,
    );

    // Test 3: Too many nested table types
    fixture.match_parse_error(
        &"local t: {a: {b: {c: {d: {e: {f: {g: {h: {i: {j: {}}}}}}}}}}}".to_string(),
        &"Exceeded allowed recursion depth; simplify your type annotation to make the code compile"
            .to_string(),
        None,
    );

    // Test 4: Too many nested parentheses in type annotation
    fixture.match_parse_error(
        &"local f: ((((((((((Fail))))))))))".to_string(),
        &"Exceeded allowed recursion depth; simplify your type annotation to make the code compile"
            .to_string(),
        None,
    );

    // Test 5: Too many nested intersection types
    fixture.match_parse_error(
        &"local t: a & (b & (c & (d & (e & (f & (g & (h & (i & (j & nil)))))))))".to_string(),
        &"Exceeded allowed recursion depth; simplify your type annotation to make the code compile"
            .to_string(),
        None,
    );
}
