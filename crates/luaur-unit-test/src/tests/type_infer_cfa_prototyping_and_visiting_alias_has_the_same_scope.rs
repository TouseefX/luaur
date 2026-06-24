//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.cfa.test.cpp:795:type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope`
//! Source: `tests/TypeInfer.cfa.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.cfa.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.cfa.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope

#[cfg(test)]
#[test]
fn type_infer_cfa_prototyping_and_visiting_alias_has_the_same_scope() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: string?)
            type Foo = number

            if typeof(x) == "string" then
                return
            end

            local foo: Foo = x
        end
    "#,
        ),
        None,
    );

    assert_eq!(
        "nil",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 8,
            column: 29,
        }))
    );
}
