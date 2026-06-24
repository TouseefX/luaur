//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1142:type_infer_cli_50041_committing_txnlog_in_apollo_client_error`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_cli_50041_committing_txnlog_in_apollo_client_error

#[cfg(test)]
#[test]
fn type_infer_cli_50041_committing_txnlog_in_apollo_client_error() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        --!nolint

        type FieldSpecifier = {
            fieldName: string,
        }

        type ReadFieldOptions = FieldSpecifier & { from: number? }

        type Policies = {
            getStoreFieldName: (self: Policies, fieldSpec: FieldSpecifier) -> string,
        }

        local Policies = {}

        local function foo(p: Policies)
        end

        function Policies:getStoreFieldName(specifier: FieldSpecifier): string
            return ""
        end

        function Policies:readField(options: ReadFieldOptions)
            local _ = self:getStoreFieldName(options)
            foo(self)
        end
    "#,
        ),
        None,
    );

    if FFlag::LuauInstantiateInSubtyping.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = concat!(
            "Expected this to be exactly 'Policies' from 'MainModule', but got 'Policies' from 'MainModule'",
            "\ncaused by:\n",
            "  Property 'getStoreFieldName' is not compatible.\n",
            "Expected this to be exactly\n\t",
            "'(Policies, FieldSpecifier) -> string'",
            "\nbut got\n\t",
            "'(Policies, FieldSpecifier & { from: number? }) -> ('a, b...)'",
            "\ncaused by:\n",
            "  Argument #2 type is not compatible.\n",
            "Expected this to be exactly\n\t",
            "'FieldSpecifier & { from: number? }'",
            "\nbut got\n\t",
            "'FieldSpecifier'",
            "\ncaused by:\n",
            "  Not all intersection parts are compatible.\n",
            "Table type 'FieldSpecifier' not compatible with type '{ from: number? }' because the former has extra field 'fieldName'"
        );
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
}
