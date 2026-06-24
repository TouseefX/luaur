//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:477:type_infer_extern_types_class_type_mismatch_with_name_conflict`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_extern_types_class_type_mismatch_with_name_conflict

#[cfg(test)]
#[test]
fn type_infer_extern_types_class_type_mismatch_with_name_conflict() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local i = ChildClass.New()
type ChildClass = { x: number }
local a: ChildClass = i
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be 'ChildClass' from 'MainModule', but got 'ChildClass' from 'Test'",
        to_string_type_error(&result.errors[0])
    );
}
