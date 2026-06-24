//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4578:autocomplete_type_function_eval_in_autocomplete`
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
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item autocomplete_type_function_eval_in_autocomplete

#[cfg(test)]
#[test]
fn autocomplete_type_function_eval_in_autocomplete() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"
type function foo(x)
    local tbl = types.newtable(nil, nil, nil)
    tbl:setproperty(types.singleton("boolean"), x)
    tbl:setproperty(types.singleton("number"), types.number)
    return tbl
end

local function test(a: foo<string>)
    return a.@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("boolean"), true);
    assert_eq!(ac.entry_map.contains_key("number"), true);
}
