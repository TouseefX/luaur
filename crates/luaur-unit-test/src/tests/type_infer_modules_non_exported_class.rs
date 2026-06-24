//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_non_exported_class() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use luaur_common::FFlag;

    let _flags = [
        ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
        ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true),
    ];
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        class Point
            public x: number
            public y: number

            function __tostring(self)
                return `Point x={self.x} y={self.y}`
            end
        end

        return {Point=Point}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local A = require(game.A)

        local a: A.Point = A.Point { x=2, y=3 }
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err = type_error_data_ref::<UnknownSymbol>(&result.errors[0])
        .expect("expected UnknownSymbol error");
    assert_eq!("A.Point", err.name());
    assert_eq!(Context::Type, err.context());
}
