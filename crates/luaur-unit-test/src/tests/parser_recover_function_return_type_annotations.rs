#[cfg(test)]
#[test]
fn parser_recover_function_return_type_annotations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "type Custom<A, B, C> = { x: A, y: B, z: C }\n\
         type Packed<A...> = { x: (A...) -> () }\n\
         type F = (number): Custom<boolean, number, string>\n\
         type G = Packed<(number): (string, number, boolean)>\n\
         local function f(x: number) -> Custom<string, boolean, number>\n\
         end",
    );

    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 3);
    assert_eq!(
        result.errors[0].get_message().as_str(),
        "Return types in function type annotations are written after '->' instead of ':'"
    );
    assert_eq!(
        result.errors[1].get_message().as_str(),
        "Return types in function type annotations are written after '->' instead of ':'"
    );
    assert_eq!(
        result.errors[2].get_message().as_str(),
        "Function return type annotations are written after ':' instead of '->'"
    );
}
