//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.cfa.test.cpp:397:type_infer_cfa_if_not_x_continue_elif_not_y_throw_elif_not_z_fallthrough`
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
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_cfa_if_not_x_continue_elif_not_y_throw_elif_not_z_fallthrough

#[cfg(test)]
#[test]
fn type_infer_cfa_if_not_x_continue_elif_not_y_throw_elif_not_z_fallthrough() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x: {{value: string?}}, y: {{value: string?}}, z: {{value: string?}})
            for i, recordX in x do
                local recordY = y[i]
                local recordZ = y[i]
                if not recordX.value then
                    continue
                elseif not recordY.value then
                    error("Y value not defined")
                elseif not recordZ.value then

                end

                local foo = recordX.value
                local bar = recordY.value
                local baz = recordZ.value
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
            line: 13,
            column: 38,
        }))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 14,
            column: 38,
        }))
    );
    assert_eq!(
        "string?",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 15,
            column: 38,
        }))
    );
}
