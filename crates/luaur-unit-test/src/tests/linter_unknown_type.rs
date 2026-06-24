//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:619:linter_unknown_type`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - translates_to -> rust_item linter_unknown_type

#[cfg(test)]
#[test]
fn linter_unknown_type() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_fun::TypeFun;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::type_aliases::props_type::Props;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().anyType;

    {
        let frontend = fixture.get_frontend();
        unfreeze(frontend.globals.global_types_mut());

        let scope = frontend.globals.global_scope();
        let scope_ptr = Arc::as_ptr(&scope) as *mut Scope;
        let mut instance_props = Props::default();
        instance_props.insert(String::from("ClassName"), Property::rw_type_id(any_type));

        let instance_table =
            TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                &instance_props,
                None,
                TypeLevel::default(),
                scope_ptr,
                TableState::Sealed,
            );
        let instance_type = frontend.globals.global_types_mut().add_type(instance_table);

        unsafe {
            (*scope_ptr).exported_type_bindings.insert(
                String::from("Part"),
                TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
                    Vec::new(),
                    instance_type,
                    None,
                ),
            );
        }
    }

    let result = fixture.lint(
        &String::from(
            r#"
local game = ...
local _e01 = type(game) == "Part"
local _e02 = typeof(game) == "Bar"
local _ok = typeof(game) == "vector"

local _o01 = type(game) == "number"
local _o02 = type(game) == "vector"
local _o03 = typeof(game) == "Part"
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(2, result.warnings[0].location.begin.line);
    assert_eq!(
        "Unknown type 'Part' (expected primitive type)",
        result.warnings[0].text
    );
    assert_eq!(3, result.warnings[1].location.begin.line);
    assert_eq!("Unknown type 'Bar'", result.warnings[1].text);
}
