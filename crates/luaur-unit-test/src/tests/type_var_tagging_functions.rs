//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:307:type_var_tagging_functions`
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
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_var_tagging_functions

#[cfg(test)]
#[test]
fn type_var_tagging_functions() {
    use alloc::vec;
    use luaur_analysis::functions::attach_tag_type::attach_tag;
    use luaur_analysis::functions::has_tag_type_alt_b::has_tag;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::records::type_pack_var::TypePackVar;

    let empty = TypePackVar::from(TypePack::new(vec![], None));
    let ftv = Type::from(FunctionType::function_type_new(&empty, &empty, None, false));
    let ty = &ftv as *const Type;

    assert!(!has_tag(ty, "foo"));
    attach_tag(ty, "foo");
    assert!(has_tag(ty, "foo"));
}
