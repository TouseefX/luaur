#[cfg(test)]
#[test]
fn parser_parse_error_missing_type_annotation() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    {
        let mut fix = Fixture::default();
        let code = alloc::string::String::from("local x:");
        let result = fix.try_parse(&code, &ParseOptions::parse_options());

        assert_eq!(result.errors.len(), 1);
        let begin = result.errors[0].get_location().begin;
        let end = result.errors[0].get_location().end;
        assert_eq!(begin.line, end.line);
        let width = end.column - begin.column;
        assert_eq!(width, 0);
        assert_eq!(&*result.errors[0].get_message(), "Expected type, got <eof>");
    }

    {
        let mut fix = Fixture::default();
        let code = alloc::string::String::from("local x:=42");
        let result = fix.try_parse(&code, &ParseOptions::parse_options());

        assert_eq!(result.errors.len(), 1);
        let begin = result.errors[0].get_location().begin;
        let end = result.errors[0].get_location().end;
        assert_eq!(begin.line, end.line);
        let width = end.column - begin.column;
        assert_eq!(width, 1);
        assert_eq!(&*result.errors[0].get_message(), "Expected type, got '='");
    }

    {
        let mut fix = Fixture::default();
        let code = alloc::string::String::from("function func():end");
        let result = fix.try_parse(&code, &ParseOptions::parse_options());

        assert_eq!(result.errors.len(), 1);
        let begin = result.errors[0].get_location().begin;
        let end = result.errors[0].get_location().end;
        assert_eq!(begin.line, end.line);
        let width = end.column - begin.column;
        assert_eq!(width, 3);
        assert_eq!(&*result.errors[0].get_message(), "Expected type, got 'end'");
    }
}
