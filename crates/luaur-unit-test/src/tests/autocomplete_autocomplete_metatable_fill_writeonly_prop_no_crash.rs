//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5176:autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_metatable_fill_writeonly_prop_no_crash() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"

local t0 = { thing = 5 }

type function evil(x)
    local tbl = types.newtable(nil, nil, nil)
    tbl:setwriteproperty(types.singleton("__index"), types.any)
    return tbl
end

type BadMTType = evil<{ thing : number}>
local function foo(t : BadMTType)
        local t2 = setmetatable({}, t)
        return t2
end

local x = foo(nil :: any)
x.@1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.is_empty());
}
