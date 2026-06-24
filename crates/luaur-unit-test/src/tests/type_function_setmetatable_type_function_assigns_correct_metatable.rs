//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1419:type_function_setmetatable_type_function_assigns_correct_metatable`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
//!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_function_setmetatable_type_function_assigns_correct_metatable

#[cfg(test)]
#[test]
fn type_function_setmetatable_type_function_assigns_correct_metatable() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::metatable_type::MetatableType;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Identity = setmetatable<{}, { __index: {} }>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let id = fixture.base.require_type_alias(&String::from("Identity"));
    assert_eq!(
        "{ @metatable { __index: {  } }, {  } }",
        to_string_type_id_to_string_options_mut(id, ToStringOptions::to_string_options(true))
    );
    let mt = unsafe { get_type_id::<MetatableType>(id).as_ref() }
        .unwrap_or_else(|| panic!("expected MetatableType, got {}", to_string_type_id(id)));
    assert_eq!("{ __index: {  } }", to_string_type_id(mt.metatable()));
}
