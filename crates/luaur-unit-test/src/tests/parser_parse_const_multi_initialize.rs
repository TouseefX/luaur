#[cfg(test)]
#[test]
fn parser_parse_const_multi_initialize() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::LuauConst2, true);

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
        r#"const a, b = 42, 32

const a, b, c = 42, f()

const a, b, c = 42, ...
"#,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    assert!(!_stat.is_null());
}
