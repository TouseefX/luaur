//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:864:type_infer_extern_types_cyclic_tables_are_assumed_to_be_compatible_with_extern_types`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
//!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record Subtyping (Analysis/include/Luau/Subtyping.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item type_infer_extern_types_cyclic_tables_are_assumed_to_be_compatible_with_extern_types

#[cfg(test)]
#[test]
fn type_infer_extern_types_cyclic_tables_are_assumed_to_be_compatible_with_extern_types() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local c = BaseClass.New()

        function requiresNothing() end

        function onTouch(other)
            requiresNothing(other:BaseMethod(0))
            print(other.BaseField)
        end

        c.Touched:Connect(onTouch)
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
