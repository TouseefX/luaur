#[cfg(test)]
#[test]
fn parser_parse_local_const() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;

    let _sff = ScopedFastFlag::new(&luaur_common::FFlag::LuauConst2, true);
    let mut fixture = Fixture::default();
    let _stat = fixture.parse("local const", &ParseOptions::default());
    assert!(!_stat.is_null());
}
