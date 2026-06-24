//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:153:type_infer_definitions_class_definitions_cannot_overload_non_function`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - translates_to -> rust_item type_infer_definitions_class_definitions_cannot_overload_non_function

#[cfg(test)]
#[test]
fn type_infer_definitions_class_definitions_cannot_overload_non_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::frontend::Frontend;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let result = unsafe {
        let target_scope = (*frontend_ptr).globals.global_scope();
        (*frontend_ptr).load_definition_file(
            &mut (*frontend_ptr).globals,
            target_scope,
            r#"
        declare class A
            X: number
            X: string
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

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, module.errors.len(), "{:?}", module.errors);
    } else {
        assert_eq!(1, module.errors.len(), "{:?}", module.errors);
    }

    let ge = match &module.errors[0].data {
        TypeErrorData::GenericError(ge) => ge,
        other => panic!("expected GenericError, got {:?}", other),
    };
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Cannot overload read type of non-function extern type member 'X'",
            ge.message()
        );
        let ge2 = match &module.errors[1].data {
            TypeErrorData::GenericError(ge) => ge,
            other => panic!("expected GenericError, got {:?}", other),
        };
        assert_eq!(
            "Cannot overload write type of non-function extern type member 'X'",
            ge2.message()
        );
    } else {
        assert_eq!(
            "Cannot overload non-function class member 'X'",
            ge.message()
        );
    }
}
