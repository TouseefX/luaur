//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:290:type_var_tagging_subextern_types`
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
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_var_tagging_subextern_types

#[cfg(test)]
#[test]
fn type_var_tagging_subextern_types() {
    use luaur_analysis::functions::attach_tag_type::attach_tag;
    use luaur_analysis::functions::has_tag_type_alt_b::has_tag;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::r#type::Type;

    let base = Type::from(ExternType {
        name: "Base".into(),
        props: Default::default(),
        parent: None,
        metatable: None,
        tags: Default::default(),
        user_data: None,
        definition_module_name: "Test".into(),
        definition_location: None,
        indexer: None,
        relation: None,
    });
    let base_id = &base as *const Type;
    let derived = Type::from(ExternType {
        name: "Derived".into(),
        props: Default::default(),
        parent: Some(base_id),
        metatable: None,
        tags: Default::default(),
        user_data: None,
        definition_module_name: "Test".into(),
        definition_location: None,
        indexer: None,
        relation: None,
    });
    let derived_id = &derived as *const Type;

    assert!(!has_tag(base_id, "foo"));
    assert!(!has_tag(derived_id, "foo"));

    attach_tag(base_id, "foo");
    assert!(has_tag(base_id, "foo"));
    assert!(has_tag(derived_id, "foo"));

    attach_tag(derived_id, "bar");
    assert!(!has_tag(base_id, "bar"));
    assert!(has_tag(derived_id, "bar"));
}
