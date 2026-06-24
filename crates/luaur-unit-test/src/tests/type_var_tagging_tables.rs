//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:274:type_var_tagging_tables`
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
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_var_tagging_tables

#[cfg(test)]
#[test]
fn type_var_tagging_tables() {
    use luaur_analysis::functions::attach_tag_type::attach_tag;
    use luaur_analysis::functions::has_tag_type_alt_b::has_tag;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::table_type::TableType;

    let ttv = Type::from(TableType::table_type());
    let ty = &ttv as *const Type;

    assert!(!has_tag(ty, "foo"));
    attach_tag(ty, "foo");
    assert!(has_tag(ty, "foo"));
}
