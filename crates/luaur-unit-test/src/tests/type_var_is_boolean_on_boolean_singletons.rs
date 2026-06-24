//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:408:type_var_is_boolean_on_boolean_singletons`
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
//!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record BooleanSingleton (Analysis/include/Luau/Type.h)
//!   - calls -> function isBoolean (Analysis/src/Type.cpp)
//!   - translates_to -> rust_item type_var_is_boolean_on_boolean_singletons

#[cfg(test)]
#[test]
fn type_var_is_boolean_on_boolean_singletons() {
    use luaur_analysis::functions::is_boolean::is_boolean;
    use luaur_analysis::records::boolean_singleton::BooleanSingleton;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::singleton_type::SingletonType;
    use luaur_analysis::type_aliases::singleton_variant::SingletonVariant;

    let true_bool = Type::from(SingletonType {
        variant: SingletonVariant::V0(BooleanSingleton { value: true }),
    });

    assert!(is_boolean(&true_bool));
}
