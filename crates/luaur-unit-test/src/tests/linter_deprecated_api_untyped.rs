//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1567:linter_deprecated_api_untyped`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function getGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
//!   - calls -> function foreach (VM/src/ltablib.cpp)
//!   - calls -> function getn (VM/src/ltablib.cpp)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item linter_deprecated_api_untyped

#[cfg(test)]
#[test]
fn linter_deprecated_api_untyped() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_global_binding::get_global_binding;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();

    {
        let frontend = fixture.get_frontend();
        let table_type = get_global_binding(&mut frontend.globals, "table");
        let table = unsafe { get_mutable_type_id::<TableType>(table_type) };
        if let Some(table) = unsafe { table.as_mut() } {
            if let Some(prop) = table.props.get_mut("foreach") {
                prop.deprecated = true;
            }
            if let Some(prop) = table.props.get_mut("getn") {
                prop.deprecated = true;
                prop.deprecated_suggestion = String::from("#");
            }
        }
    }

    let result = fixture.base.lint(
        &String::from(
            r#"
-- TODO
return function ()
    print(table.getn({}))
    table.foreach({}, function() end)
    print(table.nogetn()) -- verify that we correctly handle non-existent members
end
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Member 'table.getn' is deprecated, use '#' instead",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Member 'table.foreach' is deprecated",
        result.warnings[1].text.as_str()
    );
}
