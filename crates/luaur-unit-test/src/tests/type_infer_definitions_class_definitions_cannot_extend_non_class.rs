//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:193:type_infer_definitions_class_definitions_cannot_extend_non_class`
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
//!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_definitions_class_definitions_cannot_extend_non_class

#[cfg(test)]
#[test]
fn type_infer_definitions_class_definitions_cannot_extend_non_class() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::frontend::Frontend;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let result = unsafe {
        let target_scope = (*frontend_ptr).globals.global_scope();
        (*frontend_ptr).load_definition_file(
            &mut (*frontend_ptr).globals,
            target_scope,
            r#"
        type NotAClass = {}

        declare class Foo extends NotAClass
        end
    "#,
            String::from("@test"),
            false,
            false,
        )
    };

    assert!(!result.success);
    assert_eq!(0, result.parse_result.errors.len());
    let module = result.module.as_ref().expect("checked definition module");
    assert_eq!(1, module.errors.len(), "{:?}", module.errors);

    let ge = match &module.errors[0].data {
        TypeErrorData::GenericError(ge) => ge,
        other => panic!("expected GenericError, got {:?}", other),
    };
    assert_eq!(
        "Cannot use non-class type 'NotAClass' as a superclass of class 'Foo'",
        ge.message()
    );
}
