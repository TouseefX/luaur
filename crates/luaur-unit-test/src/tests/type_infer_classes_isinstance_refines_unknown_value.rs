//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:192:type_infer_classes_isinstance_refines_unknown_value`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_classes_isinstance_refines_unknown_value

#[cfg(test)]
#[test]
fn type_infer_classes_isinstance_refines_unknown_value() {
    use crate::records::classes_fixture::ClassesFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::LuauIntegerType2, true);
    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
class Point
    public x
end

local function f(v: unknown)
    if class.isinstance(v, Point) then
        local s = v
    else
        local s = v
    end
end
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Point",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 7,
            column: 18
        }))
    );
    assert_eq!(
        "((userdata & ~Point) | boolean | buffer | function | integer | number | string | table | thread)?",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position { line: 9, column: 18 })
        )
    );
}
