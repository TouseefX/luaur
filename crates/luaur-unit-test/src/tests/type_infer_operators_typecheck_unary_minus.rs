//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:567:type_infer_operators_typecheck_unary_minus`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> macro tostring (VM/src/lvm.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_operators_typecheck_unary_minus

#[cfg(test)]
#[test]
fn type_infer_operators_typecheck_unary_minus() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::generic_error::GenericError;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_analysis::records::uninhabited_type_function::UninhabitedTypeFunction;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local foo
        local mt = {}

        mt.__unm = function(val): string
            return tostring(val.value) .. "test"
        end

        foo = setmetatable({
            value = 10
        }, mt)

        local a = -foo

        local b = 1+-1

        local bar = {
            value = 10
        }
        local c = -bar -- disallowed
    "#,
        ),
        None,
    );

    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);

        let utf = type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0])
            .expect("expected UninhabitedTypeFunction");
        assert_eq!("unm<bar>", to_string_type_id(utf.ty()));

        let tm =
            type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
        assert_eq!("bar", to_string_type_id(tm.given_type));
        assert_eq!("number", to_string_type_id(tm.wanted_type));
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);

        let gen =
            type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
        assert_eq!(
            "Unary operator '-' not supported by type 'bar'",
            gen.message()
        );
    }
}
