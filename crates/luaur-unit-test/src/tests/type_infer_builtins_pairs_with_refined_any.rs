//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1894:type_infer_builtins_pairs_with_refined_any`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_builtins_pairs_with_refined_any

#[cfg(test)]
#[test]
fn type_infer_builtins_pairs_with_refined_any() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local t: any = {"hello", "world"}
        if type(t) == "table" and pairs(t) then
	        local foo, bar, lorem = pairs(t)
            local _ = foo
            local _ = bar
            local _ = lorem
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "({+ [unknown]: unknown +}, unknown?) -> (unknown?, unknown)",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 5,
            column: 23
        }))
    );
    assert_eq!(
        "{+ [unknown]: unknown +}",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 6,
            column: 23
        }))
    );
    assert_eq!(
        "nil",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 7,
            column: 23
        }))
    );
}
