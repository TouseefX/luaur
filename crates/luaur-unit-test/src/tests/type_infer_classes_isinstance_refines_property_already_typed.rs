//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:296:type_infer_classes_isinstance_refines_property_already_typed`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_classes_isinstance_refines_property_already_typed

#[cfg(test)]
#[test]
fn type_infer_classes_isinstance_refines_property_already_typed() {
    use crate::records::classes_fixture::ClassesFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
class Point
    public x
end

local function f(t: { x: Point })
    if class.isinstance(t.x, Point) then
        local s = t.x
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
            column: 20
        }))
    );
}
