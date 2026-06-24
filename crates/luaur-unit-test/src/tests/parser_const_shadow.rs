#[cfg(test)]
#[test]
fn parser_const_shadow() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauConst2;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);

    let source = alloc::string::String::from(
        "const a = 42\n\
         const a = 43\n\
         \n\
         do\n\
             const a = 44\n\
             do\n\
                 local a = 44.1\n\
                 do\n\
                     const a = 44.2\n\
                 end\n\
                 a = 44.3\n\
             end\n\
         end\n\
         \n\
         function f()\n\
             const a = 45\n\
             local a = 46\n\
             return function(x) a = x end\n\
         end",
    );
    let _stat = fixture.parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    assert!(!_stat.is_null());
}
