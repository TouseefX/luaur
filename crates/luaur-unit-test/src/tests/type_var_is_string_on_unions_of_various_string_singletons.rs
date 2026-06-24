//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:389:type_var_is_string_on_unions_of_various_string_singletons`
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
//!   - type_ref -> record StringSingleton (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_var_is_string_on_unions_of_various_string_singletons

#[cfg(test)]
#[test]
fn type_var_is_string_on_unions_of_various_string_singletons() {
    use alloc::string::String;
    use alloc::vec;
    use luaur_analysis::functions::is_string::is_string;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::singleton_type::SingletonType;
    use luaur_analysis::records::string_singleton::StringSingleton;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::singleton_variant::SingletonVariant;

    let hello_string = Type::from(SingletonType {
        variant: SingletonVariant::V1(StringSingleton {
            value: String::from("hello"),
        }),
    });
    let bye_string = Type::from(SingletonType {
        variant: SingletonVariant::V1(StringSingleton {
            value: String::from("bye"),
        }),
    });
    let union_ = Type::from(UnionType {
        options: vec![&hello_string, &bye_string],
    });

    assert!(is_string(&union_));
}
