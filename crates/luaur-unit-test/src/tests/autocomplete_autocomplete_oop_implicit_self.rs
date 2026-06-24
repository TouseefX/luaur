//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3057:autocomplete_autocomplete_oop_implicit_self`
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
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_oop_implicit_self

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_oop_implicit_self() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"
--!strict
local Class = {}
Class.__index = Class
type Class = typeof(setmetatable({} :: { x: number }, Class))
function Class.new(x: number): Class
    return setmetatable({x = x}, Class)
end
function Class.getx(self: Class)
    return self.x
end
function test()
    local c = Class.new(42)
    local n = c:@1
    print(n)
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("getx"));
}
