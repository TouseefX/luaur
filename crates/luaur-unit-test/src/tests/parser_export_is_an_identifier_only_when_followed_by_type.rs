#[cfg(test)]
#[test]
fn parser_export_is_an_identifier_only_when_followed_by_type() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let mut fixture = Fixture::default();
    let _sff = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, false);

    let result = fixture.try_parse(
        &alloc::string::String::from("export function a() end"),
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        result.errors.first().unwrap().get_message().as_str(),
        "Incomplete statement: expected assignment or a function call"
    );
}
