//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:106:type_infer_classes_box_point_no_eq`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotCompareUnrelatedTypes (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_classes_box_point_no_eq

#[cfg(test)]
#[test]
fn type_infer_classes_box_point_no_eq() {
    use crate::records::classes_fixture::ClassesFixture;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
class Point
    public x
    public y
end


class Box
    public x
end

local p1 = Point { x = 1, y = 2 }
local p2 = Box { x = 1 }
local _ = p1 == p1
-- This one too
local _ = p1 ~= p2
local _ = Box == Box
-- This line should error...
local _ = Point ~= Box
"#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(
        matches!(
            &result.errors[0].data,
            TypeErrorData::CannotCompareUnrelatedTypes(_)
        ),
        "{:?}",
        result.errors[0]
    );
    assert!(
        matches!(
            &result.errors[1].data,
            TypeErrorData::CannotCompareUnrelatedTypes(_)
        ),
        "{:?}",
        result.errors[1]
    );
    assert_eq!(15, result.errors[0].location.begin.line);
    assert_eq!(18, result.errors[1].location.begin.line);
}
