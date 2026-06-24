//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_export_class() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _flags = [
        ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
        ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
        ScopedFastFlag::new(&FFlag::LuauConst2, true),
        ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
        ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        export class Point
            public x: number
            public y: number

            function __tostring(self)
                return `Point x={self.x} y={self.y}`
            end
        end
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local A = require(game.A)

        local a: A.Point = A.Point { x=2, y=3 }

        local x, y = a.x, a.y
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "number",
        to_string_type_id(
            fixture
                .base
                .require_type_module_name_string("game/B", &String::from("x"))
        )
    );
    assert_eq!(
        "number",
        to_string_type_id(
            fixture
                .base
                .require_type_module_name_string("game/B", &String::from("y"))
        )
    );
}
