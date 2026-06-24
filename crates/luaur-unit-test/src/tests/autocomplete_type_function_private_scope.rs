//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4547:autocomplete_type_function_private_scope`
//! Source: `tests/Autocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Autocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Autocomplete.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item autocomplete_type_function_private_scope

#[cfg(test)]
#[test]
fn autocomplete_type_function_private_scope() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b;
    use luaur_analysis::records::binding::Binding;
    use luaur_ast::records::location::Location;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    {
        let frontend =
            fixture.base.get_frontend() as *mut luaur_analysis::records::frontend::Frontend;
        unsafe {
            let any_type = (*(*frontend).builtin_types).anyType;
            let binding = || Binding {
                type_id: any_type,
                location: Location::default(),
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
            };

            add_global_binding_builtin_definitions_alt_b(
                &mut (*frontend).globals,
                "thisAlsoShouldNotBeThere",
                binding(),
            );
            add_global_binding_builtin_definitions_alt_b(
                &mut (*frontend).globals_for_autocomplete,
                "thisAlsoShouldNotBeThere",
                binding(),
            );
        }
    }

    fixture.base.check(&String::from(
        r#"
local function thisShouldNotBeThere() end

type function thisShouldBeThere() end

type function foo()
    this@1
end

this@2
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("thisShouldNotBeThere"), false);
    assert_eq!(ac.entry_map.contains_key("thisAlsoShouldNotBeThere"), false);
    assert_eq!(ac.entry_map.contains_key("thisShouldBeThere"), true);

    let ac = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("thisShouldNotBeThere"), true);
    assert_eq!(ac.entry_map.contains_key("thisAlsoShouldNotBeThere"), true);
    assert_eq!(ac.entry_map.contains_key("thisShouldBeThere"), false);
}
