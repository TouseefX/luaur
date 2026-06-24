//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:394:type_infer_definitions_documentation_symbols_dont_attach_to_persistent_types`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_definitions_documentation_symbols_dont_attach_to_persistent_types

#[cfg(test)]
#[test]
fn type_infer_definitions_documentation_symbols_dont_attach_to_persistent_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        export type Evil = string
    "#,
        ),
        false,
    );

    let frontend = fixture.get_frontend();
    let ty = frontend
        .globals
        .global_scope()
        .lookup_type(&String::from("Evil"))
        .expect("Evil type binding");

    assert_eq!(None, unsafe { (*ty.r#type()).documentation_symbol.clone() });
}
