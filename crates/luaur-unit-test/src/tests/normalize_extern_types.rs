//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:310:normalize_extern_types`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - calls -> function createSomeExternTypes (tests/Fixture.cpp)
//!   - calls -> method NormalizeFixture::getFrontend (tests/Normalize.test.cpp)
//!   - calls -> function main (tests/main.cpp)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item normalize_extern_types

#[cfg(test)]
#[test]
fn normalize_extern_types() {
    use crate::functions::create_some_extern_types::create_some_extern_types;
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;
    use alloc::sync::Arc;
    use luaur_analysis::records::scope::Scope;

    let mut fixture = IsSubtypeFixture::default();

    {
        let frontend = fixture.base.get_frontend();
        create_some_extern_types(frontend);
    }

    fixture
        .base
        .check_string_optional_frontend_options(&String::from(""), None);

    let (p, c, u) = {
        let frontend = fixture.base.get_frontend();
        let module_scope = frontend.globals.global_scope();
        let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

        unsafe {
            (
                (*module_scope_ptr)
                    .exported_type_bindings
                    .get("Parent")
                    .expect("Parent exported type binding")
                    .r#type(),
                (*module_scope_ptr)
                    .exported_type_bindings
                    .get("Child")
                    .expect("Child exported type binding")
                    .r#type(),
                (*module_scope_ptr)
                    .exported_type_bindings
                    .get("Unrelated")
                    .expect("Unrelated exported type binding")
                    .r#type(),
            )
        }
    };

    assert!(fixture.is_subtype(c, p));
    assert!(!fixture.is_subtype(p, c));
    assert!(!fixture.is_subtype(u, p));
    assert!(!fixture.is_subtype(p, u));
}
