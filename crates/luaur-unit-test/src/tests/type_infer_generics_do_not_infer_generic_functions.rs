//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1502:type_infer_generics_do_not_infer_generic_functions`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_generics_do_not_infer_generic_functions

#[cfg(test)]
#[test]
fn type_infer_generics_do_not_infer_generic_functions() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = if !FFlag::DebugLuauForceOldSolver.get() {
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            local function sum<T>(x: T, y: T, z: (T, T) -> T) return z(x, y) end

            local function sumrec(f: typeof(sum))
                return sum(2, 3, function<X>(g: X, h: X): add<X, X> return g + h end)
            end

            local b = sumrec(sum) -- ok
            local c = sumrec(
                function(d, e, f)
                    return f(d, e)
                end
            ) -- type binders are not inferred
        "#,
            ),
            None,
        );

        assert_eq!(
            "number",
            to_string_type_id(fixture.base.require_type_string(&String::from("b")))
        );
        assert_eq!(
            "<T>(T, T, (T, T) -> T) -> T",
            to_string_type_id(fixture.base.require_type_string(&String::from("sum")))
        );
        assert_eq!(
            "<T>(T, T, (T, T) -> T) -> T",
            to_string_type_id(fixture.base.require_type_at_position_position(Position {
                line: 7,
                column: 29
            }))
        );
        result
    } else {
        fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            local function sum<a>(x: a, y: a, f: (a, a) -> a) return f(x, y) end

            local function sumrec(f: typeof(sum))
                return sum(2, 3, function(a, b) return a + b end)
            end

            local b = sumrec(sum) -- ok
            local c = sumrec(function(x, y, f) return f(x, y) end) -- type binders are not inferred
        "#,
            ),
            None,
        )
    };

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
