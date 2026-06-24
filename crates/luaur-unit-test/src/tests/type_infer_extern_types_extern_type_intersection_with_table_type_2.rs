//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:1226:type_infer_extern_types_extern_type_intersection_with_table_type_2`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item type_infer_extern_types_extern_type_intersection_with_table_type_2

#[cfg(test)]
#[test]
fn type_infer_extern_types_extern_type_intersection_with_table_type_2() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _normalize = ScopedFastFlag::new(&FFlag::LuauExternTypesNormalizeWithShapes, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.load_definition(
        &String::from(
            r#"
        declare extern type Instance with
            name: string
        end

        declare extern type WithBrushes extends Instance with
            brushes: Instance
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function take(thing: Instance & { brushes: Instance })
            print(thing)
            print(thing.brushes.name)
        end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "Instance & { brushes: Instance }",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 2,
            column: 18
        }))
    );
}
