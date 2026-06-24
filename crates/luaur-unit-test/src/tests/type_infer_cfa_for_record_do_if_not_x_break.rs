//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.cfa.test.cpp:471:type_infer_cfa_for_record_do_if_not_x_break`
//! Source: `tests/TypeInfer.cfa.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.cfa.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.cfa.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_cfa_for_record_do_if_not_x_break

#[cfg(test)]
#[test]
fn type_infer_cfa_for_record_do_if_not_x_break() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: {{value: string?}})
            for _, record in x do
                do
                    if not record.value then
                        break
                    end
                end

                local foo = record.value
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 9,
            column: 38,
        }))
    );
}
