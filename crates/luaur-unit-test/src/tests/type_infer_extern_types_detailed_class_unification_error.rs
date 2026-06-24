//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:443:type_infer_extern_types_detailed_class_unification_error`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_extern_types_detailed_class_unification_error

#[cfg(test)]
#[test]
fn type_infer_extern_types_detailed_class_unification_error() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local function foo(v)
    return v.X :: number + string.len(v.Y)
end

local a: Vector2
local b = foo
b(a)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected =
            "Expected this to be '{ read X: unknown, read Y: string }', but got 'Vector2'; \n\
accessing `Y` results in `number` in the latter type and `string` in the former type, \
and `number` is not a subtype of `string`";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        let expected = "Expected this to be '{- X: number, Y: string -}', but got 'Vector2'\n\
caused by:\n  Property 'Y' is not compatible.\n\
Expected this to be 'string', but got 'number'";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
