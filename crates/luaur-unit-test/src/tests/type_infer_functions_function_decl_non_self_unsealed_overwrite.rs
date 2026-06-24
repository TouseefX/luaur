//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1745:type_infer_functions_function_decl_non_self_unsealed_overwrite`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record WhereClauseNeeded (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_function_decl_non_self_unsealed_overwrite

#[cfg(test)]
#[test]
fn type_infer_functions_function_decl_non_self_unsealed_overwrite() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::where_clause_needed::WhereClauseNeeded;
    use luaur_common::FFlag;

    let _check_function_statement_types =
        ScopedFastFlag::new(&FFlag::LuauCheckFunctionStatementTypes, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local t = { f = nil :: ((x: number) -> number)? }

function t.f(x: string): string -- 1st error: new function value type is incompatible
    return x .. "asd"
end

t.f = function(x)
    print(x + 5)
    return x .. "asd" -- 2nd error: we know that return type is a number, not a string
end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(
            result
                .errors
                .iter()
                .any(|error| type_error_data_ref::<WhereClauseNeeded>(error).is_some()),
            "{:?}",
            result.errors
        );
    } else {
        assert_eq!(
            r#"Expected this to be
	'((number) -> number)?'
but got
	'(string) -> string'
caused by:
  None of the union options are compatible. For example:
Expected this to be
	'(number) -> number'
but got
	'(string) -> string'
caused by:
  Argument #1 type is not compatible.
Expected this to be 'string', but got 'number'"#,
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[1])
        );
    }
}
