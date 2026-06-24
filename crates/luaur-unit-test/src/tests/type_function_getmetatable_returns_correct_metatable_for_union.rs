//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1548:type_function_getmetatable_returns_correct_metatable_for_union`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method TFFixture::getBuiltins (tests/TypeFunction.test.cpp)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_function_getmetatable_returns_correct_metatable_for_union

#[cfg(test)]
#[test]
fn type_function_getmetatable_returns_correct_metatable_for_union() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
    use luaur_analysis::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Identity = setmetatable<{}, {}>
        type Metatable = getmetatable<string | Identity>
        type IntersectMetatable = getmetatable<string & Identity>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let builtins = unsafe { &*fixture.base.builtin_types };
    let string_metatable =
        get_metatable_type_id_not_null_builtin_types(builtins.string_type(), builtins)
            .expect("expected string metatable");
    let mut arena = TypeArena::default();

    let expected_union = arena.add_type(UnionType {
        options: alloc::vec![string_metatable, builtins.empty_table_type()],
    });
    assert_eq!(
        to_string_type_id_to_string_options_mut(
            expected_union,
            ToStringOptions::to_string_options(true)
        ),
        to_string_type_id_to_string_options_mut(
            fixture.base.require_type_alias(&String::from("Metatable")),
            ToStringOptions::to_string_options(true)
        )
    );

    let expected_intersection = arena.add_type(IntersectionType {
        parts: alloc::vec![string_metatable, builtins.empty_table_type()],
    });
    assert_eq!(
        to_string_type_id_to_string_options_mut(
            expected_intersection,
            ToStringOptions::to_string_options(true)
        ),
        to_string_type_id_to_string_options_mut(
            fixture
                .base
                .require_type_alias(&String::from("IntersectMetatable")),
            ToStringOptions::to_string_options(true)
        )
    );
}
