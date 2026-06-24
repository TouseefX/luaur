//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3865:autocomplete_autocomplete_response_perf_1`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function formatAppend (Common/src/StringUtils.cpp)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_response_perf_1

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_response_perf_1() {
    use crate::records::ac_fixture::AcFixture;
    use alloc::format;
    use alloc::string::String;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let parts = 100;
    let mut source = String::new();

    for i in 0..parts {
        source.push_str(&format!("type T{} = {{ f{}: number }}\n", i, i));
    }

    source.push_str("type Instance = { new: (('s0', extra: Instance?) -> T0)");

    for i in 1..parts {
        source.push_str(&format!(" & (('s{}', extra: Instance?) -> T{})", i, i));
    }

    source.push_str(" }\n");
    source.push_str("local Instance: Instance = {} :: any\n");
    source.push_str("local function c(): boolean return t@1 end\n");

    let mut fixture = AcFixture::default();
    fixture.base.check(&source);

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("true"));
    assert!(ac.entry_map.contains_key("Instance"));
}
