#[cfg(test)]
#[test]
fn parser_parse_const_call() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauConst2;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);

    let source =
        alloc::string::String::from("local const = function(t) return t end\nconst { a = \"a\" }");
    let _stat = fixture.parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
}
