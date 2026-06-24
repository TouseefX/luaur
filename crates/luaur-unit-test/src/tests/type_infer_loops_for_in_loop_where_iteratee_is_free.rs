//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:646:type_infer_loops_for_in_loop_where_iteratee_is_free`
//! Source: `tests/TypeInfer.loops.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.loops.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.loops.test.cpp
//! - outgoing:
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item type_infer_loops_for_in_loop_where_iteratee_is_free

#[cfg(test)]
#[test]
fn type_infer_loops_for_in_loop_where_iteratee_is_free() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        function _:_(...)
        end

        repeat
            if _ then
            else
                _ = ...
            end
        until _

        for _ in _() do
        end
    "#,
        ),
        None,
    );
}
