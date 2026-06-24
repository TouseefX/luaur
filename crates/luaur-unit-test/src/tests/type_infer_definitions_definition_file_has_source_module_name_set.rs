//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:538:type_infer_definitions_definition_file_has_source_module_name_set`
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
//!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_definitions_definition_file_has_source_module_name_set

#[cfg(test)]
#[test]
fn type_infer_definitions_definition_file_has_source_module_name_set() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::extern_type::ExternType;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.load_definition(
        &String::from(
            r#"
        declare class Foo
        end
    "#,
        ),
        false,
    );

    assert!(result.success);
    assert_eq!("@test", result.source_module.name);
    assert_eq!("@test", result.source_module.human_readable_name);

    let frontend = fixture.get_frontend();
    let foo_ty = frontend
        .globals
        .global_scope()
        .lookup_type(&String::from("Foo"))
        .expect("Foo type binding");
    let etv = unsafe { get_type_id::<ExternType>(foo_ty.r#type()) };
    assert!(!etv.is_null(), "expected Foo extern type");
    assert_eq!("@test", unsafe { &(*etv).definition_module_name });
}
