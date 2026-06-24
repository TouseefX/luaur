//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3895:autocomplete_autocomplete_subtyping_recursion_limit`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function formatAppend (Common/src/StringUtils.cpp)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_subtyping_recursion_limit

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_subtyping_recursion_limit() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::format;
    use alloc::string::String;
    use luaur_common::{DFInt, FFlag, FInt};

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let _type_infer_recursion_limit = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 10);
    let _subtyping_recursion_limit = ScopedFastInt::new(&DFInt::LuauSubtypingRecursionLimit, 10);

    let parts = 100;
    let mut source = String::new();

    source.push_str("function f()\n");

    let mut prefix = String::new();
    for i in 0..parts {
        prefix.push_str(&format!("(nil|({{a{}:number}}&", i));
    }
    prefix.push_str(&format!("(nil|{{a{}:number}})", parts));
    for _ in 0..parts {
        prefix.push_str("))");
    }

    source.push_str("local x1 : ");
    source.push_str(&prefix);
    source.push('\n');
    source.push_str("local y : {a1:number} = x@1\n");
    source.push_str("end\n");

    let mut fixture = AcFixture::default();
    fixture.base.check(&source);

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("true"));
    assert!(ac.entry_map.contains_key("x1"));
}
