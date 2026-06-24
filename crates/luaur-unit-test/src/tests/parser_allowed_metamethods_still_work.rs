#[cfg(test)]
#[test]
fn parser_allowed_metamethods_still_work() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let mut fix = Fixture::default();
    let code = r"class Foo
    function __tostring(self) end
    function __add(self, other) end
    function __eq(self, other) end
    -- Silly, but allowed.
    function _(self) end
end";

    let _flag = crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
        &DebugLuauUserDefinedClasses,
        true,
    );
    let result = fix.try_parse(
        &alloc::string::String::from(code),
        &ParseOptions::parse_options(),
    );

    assert_eq!(result.errors.len(), 0);
}
