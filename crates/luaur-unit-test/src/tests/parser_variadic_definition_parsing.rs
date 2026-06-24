#[cfg(test)]
#[test]
fn parser_variadic_definition_parsing() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse_ex(
        &alloc::string::String::from(
            "declare function foo(...: string): ...string\n\
             declare class Foo\n\
                 function a(self, ...: string): ...string\n\
             end",
        ),
        &ParseOptions::default(),
    );
    let root = unsafe { &*stat.root };
    assert!(!root.body.data.is_null());

    fixture.match_parse_error(
        &alloc::string::String::from("declare function foo(...)"),
        &alloc::string::String::from("All declaration parameters must be annotated"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("declare class Foo function a(self, ...) end"),
        &alloc::string::String::from(
            "All declaration parameters aside from 'self' must be annotated",
        ),
        None,
    );
}
