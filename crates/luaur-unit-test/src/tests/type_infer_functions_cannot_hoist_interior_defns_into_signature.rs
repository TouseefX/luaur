//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:107:type_infer_functions_cannot_hoist_interior_defns_into_signature`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_cannot_hoist_interior_defns_into_signature

#[cfg(test)]
#[test]
fn type_infer_functions_cannot_hoist_interior_defns_into_signature() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: T)
            type T = number
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 1,
                column: 28
            },
            end: Position {
                line: 1,
                column: 29
            }
        },
        result.errors[0].location
    );
    assert_eq!(String::from("MainModule"), result.errors[0].module_name);
    let err =
        type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("T", err.name());
    assert_eq!(Context::Type, err.context());
}
