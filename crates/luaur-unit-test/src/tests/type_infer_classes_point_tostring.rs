//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:63:type_infer_classes_point_tostring`
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
//!   - calls -> macro tostring (VM/src/lvm.h)
//!   - translates_to -> rust_item type_infer_classes_point_tostring

#[cfg(test)]
#[test]
fn type_infer_classes_point_tostring() {
    use crate::records::classes_fixture::ClassesFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
class Point
    public x
    public y
    function __tostring(self)
        return `Point(x={self.x}, y={self.y})`
    end
end

local p = Point { x = 1, y = 2 }
local _ = tostring(p)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
