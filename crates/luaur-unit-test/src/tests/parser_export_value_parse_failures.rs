#[cfg(test)]
#[test]
fn parser_export_value_parse_failures() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;
    use luaur_common::FFlag::LuauConst2;
    use luaur_common::FFlag::LuauExportValueSyntax;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);
    let _sff_debug_luau_user_defined_classes =
        ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let sources = [
        String::from("\nexport foo = 5\n    "),
        String::from("\nexport foo\n    "),
        String::from("\nfunction foo()\nend\nexport foo\n    "),
        String::from("\nexport local function foo()\nend\n    "),
    ];

    for source in sources.iter() {
        let _result = fixture.try_parse(
            source,
            &luaur_ast::records::parse_options::ParseOptions::default(),
        );
    }

    let duplicate_export = fixture.try_parse(
        &String::from("\nexport local foo = 1\nexport local foo = 2\n    "),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    let first_error_message = duplicate_export.errors.first().unwrap().get_message();
    assert!(first_error_message.contains("foo"));

    fixture.match_parse_error(
        &String::from("\nexport local answer = 42\nreturn {answer = answer}\n    "),
        &String::from(
            "Exporting values is not compatible with top-level return (export/return conflict)",
        ),
        None,
    );

    fixture.match_parse_error(
        &String::from("\nif skip then\n    return\nend\n\nexport local answer = 42\n    "),
        &String::from(
            "Exporting values is not compatible with top-level return (export/return conflict)",
        ),
        None,
    );

    fixture.match_parse_error(
        &String::from("\nexport class Player\n    public health: number\n    \n    function setHealth(self, health: number)\n        self.health = health\n        return self\n    end\n\n    function getHealth(self): number\n        return self.health\n    end\nend\n\nreturn Player {health = 100}\n    "),
        &String::from("Exporting values is not compatible with top-level return (export/return conflict)"),
        None,
    );

    let block_sources = [
        String::from("\nif true then\n    export local insideIf = 1\nend\n    "),
        String::from("\ndo\n    export const insideDo = 1\nend\n    "),
        String::from("\nwhile true do\n    export local insideWhile = 1\nend\n    "),
        String::from("\nrepeat\n    export local insideRepeat = 1\nuntil true\n    "),
        String::from("\nfor i = 1, 1 do\n    export local insideFor = i\nend\n    "),
        String::from("\nlocal function test()\n    export local insideFunction = 1\nend\n    "),
    ];

    for source in block_sources.iter() {
        fixture.match_parse_error(
            source,
            &String::from("'export' may only be applied to top-level statements"),
            None,
        );
    }
}
