//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6528:type_infer_tables_bidirectional_inference_variadic_type_pack`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item type_infer_tables_bidirectional_inference_variadic_type_pack

#[cfg(test)]
#[test]
fn type_infer_tables_bidirectional_inference_variadic_type_pack() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
        ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local foo: { (...string) -> () } = {
            function (foobar)
                print(foobar)
            end
        }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position::new(3, 24))
        )
    );
}
