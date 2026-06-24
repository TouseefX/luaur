//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:316:type_var_tagging_props`
//! Source: `tests/TypeVar.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeVar.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeVar.test.cpp
//! - outgoing:
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_var_tagging_props

#[cfg(test)]
#[test]
fn type_var_tagging_props() {
    use luaur_analysis::functions::attach_tag_type_alt_b::attach_tag_property_string;
    use luaur_analysis::functions::has_tag_type_alt_c::has_tag_property_string;
    use luaur_analysis::records::property_type::Property;

    let mut prop = Property::default();
    assert!(!has_tag_property_string(&prop, "foo"));
    attach_tag_property_string(&mut prop, "foo");
    assert!(has_tag_property_string(&prop, "foo"));
}
