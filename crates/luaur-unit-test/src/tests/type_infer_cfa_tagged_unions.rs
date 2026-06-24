//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.cfa.test.cpp:865:type_infer_cfa_tagged_unions`
//! Source: `tests/TypeInfer.cfa.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.cfa.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.cfa.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_cfa_tagged_unions

#[cfg(test)]
#[test]
fn type_infer_cfa_tagged_unions() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Ok<T> = { tag: "ok", value: T }
        type Err<E> = { tag: "err", error: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function map<T, U, E>(result: Result<T, E>, f: (T) -> U): Result<U, E>
            if result.tag == "ok" then
                local tag = result.tag
                local val = result.value

                return { tag = "ok", value = f(result.value) }
            end

            local tag = result.tag
            local err = result.error

            return result
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "T",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 8,
            column: 35,
        }))
    );
    assert_eq!(
        "E",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 14,
            column: 31,
        }))
    );
    assert_eq!(
        "Err<E>",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 16,
            column: 19,
        }))
    );
}
