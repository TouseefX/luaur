//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:82:type_infer_classes_point_eq_mm`
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
//!   - translates_to -> rust_item type_infer_classes_point_eq_mm

#[cfg(test)]
#[test]
fn type_infer_classes_point_eq_mm() {
    use crate::records::classes_fixture::ClassesFixture;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
class Point
    public x
    public y

    function __eq(self, other)
        return self.x == other.x and self.y == other.y
    end
    function zero()
        return Point { x = 0, y = 0 }
    end
end

local p1 = Point { x = 1, y = 2 }
local p2 = Point { x = 1, y = 2 }
local _ = p1 == p2
local _ = p1 ~= Point.zero()
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
