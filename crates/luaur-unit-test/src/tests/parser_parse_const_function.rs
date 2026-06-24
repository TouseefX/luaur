#[cfg(test)]
#[test]
fn parser_parse_const_function() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauConst2;

    let mut fixture = Fixture::default();
    let _sff = ScopedFastFlag::new(&LuauConst2, true);

    let source = alloc::string::String::from("const function f() return 42 end");
    let result = fixture.parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert!(!result.is_null());
}
