//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:153:type_infer_classes_class_structure`
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
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_classes_class_structure

#[cfg(test)]
#[test]
fn type_infer_classes_class_structure() {
    use crate::records::classes_fixture::ClassesFixture;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = ClassesFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
class Point
    public x
    public y

    function magnitude(self)
        return sqrt(self.x * self.x + self.y * self.y)
    end

    function zero()
        return Point { x = 0, y = 0 }
    end

    function __tostring(self)
        return `Point(x={self.x}, y={self.y})`
    end

end

local p = Point
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = fixture.base.require_type_string(&String::from("p"));
    let et = unsafe { get_type_id::<ExternType>(t).as_ref() }.expect("expected ExternType");
    assert_eq!(
        Some(unsafe { (*fixture.base.builtin_types).classType }),
        et.parent
    );
    let metatable = et.metatable.expect("expected class metatable");
    assert!(et.props.contains_key("zero"));

    let cobjmeta = unsafe { get_type_id::<TableType>(follow_type_id(metatable)).as_ref() }
        .expect("expected class metatable table");
    assert!(cobjmeta.props.contains_key("__call"));
}
