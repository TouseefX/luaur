#[cfg(test)]
#[test]
fn parser_missing_declaration_prop() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from(
            "\n        declare class Foo\n            a: number,\n        end\n    ",
        ),
        &alloc::string::String::from("Expected identifier when parsing property name, got ','"),
        None,
    );
}
