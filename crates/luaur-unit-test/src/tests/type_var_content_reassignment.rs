//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:433:type_var_content_reassignment`
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
//!   - type_ref -> record AnyType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_var_content_reassignment

#[cfg(test)]
#[test]
fn type_var_content_reassignment() {
    use alloc::string::String;
    use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::any_type::AnyType;
    use luaur_analysis::records::builtin_types::BuiltinTypes;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_level::TypeLevel;

    let mut my_any = Type::from(AnyType::default());
    my_any.persistent = true;
    my_any.documentation_symbol = Some(String::from("@global/any"));

    let mut arena = TypeArena::default();
    let builtin_types = BuiltinTypes::new();
    let future_any =
        arena.fresh_type_not_null_builtin_types_type_level(&builtin_types, TypeLevel::default());

    unsafe {
        (*as_mutable_type_id(future_any)).reassign(&my_any);

        assert!(!get_type_id::<AnyType>(future_any).is_null());
        assert!(!(*future_any).persistent);
        assert_eq!(
            (*future_any).documentation_symbol.as_deref(),
            Some("@global/any")
        );
        assert_eq!((*future_any).owning_arena, &mut arena as *mut TypeArena);
    }
}
