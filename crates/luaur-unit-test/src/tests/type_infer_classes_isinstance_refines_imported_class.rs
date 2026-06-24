//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:314:type_infer_classes_isinstance_refines_imported_class`
//! Source: `tests/TypeInfer.classes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.classes.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.classes.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method ClassesFixture::getFrontend (tests/TypeInfer.classes.test.cpp)
//!   - translates_to -> rust_item type_infer_classes_isinstance_refines_imported_class

#[cfg(test)]
#[test]
fn type_infer_classes_isinstance_refines_imported_class() {
    use crate::records::classes_fixture::ClassesFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _flags = [
        ScopedFastFlag::new(&FFlag::LuauConst2, true),
        ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
        ScopedFastFlag::new(&FFlag::LuauExportValueTypecheck, true),
    ];

    let mut fixture = ClassesFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        export class Point
            public x: number
        end
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local A = require(game.A)

        local x : unknown = (A.Point {} ) :: any
        if class.isinstance(x, A.Point) then
            local y = x
        end
    "#,
        ),
    );

    fixture.get_frontend();
    let module_b = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);

    assert_eq!(0, module_b.errors.len(), "{:?}", module_b.errors);
    assert_eq!(
        "Point",
        to_string_type_id(fixture.base.require_type_at_position_module_name_position(
            "game/B",
            Position {
                line: 5,
                column: 22
            }
        ))
    );
}
