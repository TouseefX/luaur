#[cfg(test)]
#[test]
fn linter_deprecated_attribute_method_declaration() {
    use crate::functions::check_deprecated_warning::check_deprecated_warning;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
declare class Foo
   @deprecated
   function bar(self, value: number) : number
end

declare Foo: {
   new: () -> Foo
}
"#,
        ),
        false,
    );

    let result = fixture.lint(
        &String::from(
            r#"
local foo = Foo.new()
print(foo:bar(2.0))
"#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    check_deprecated_warning(
        &result.warnings[0],
        Position::new(2, 6),
        Position::new(2, 13),
        "Member 'bar' is deprecated",
    );
}
