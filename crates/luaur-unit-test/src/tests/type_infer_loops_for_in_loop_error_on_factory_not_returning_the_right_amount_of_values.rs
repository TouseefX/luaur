//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:333:type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values

#[cfg(test)]
#[test]
fn type_infer_loops_for_in_loop_error_on_factory_not_returning_the_right_amount_of_values() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::count_mismatch::{CountMismatch, CountMismatchContext};
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function hasDivisors(value: number, table)
            return false
        end

        function prime_iter(state, index)
            while hasDivisors(index, state) do
                index += 1
            end

            state[index] = true
            return index
        end

        function primes1()
            return prime_iter, {}
        end

        function primes2()
            return prime_iter, {}, ""
        end

        function primes3()
            return prime_iter, {}, 2
        end

        for p in primes1() do print(p) end -- mismatch in argument count

        for p in primes2() do print(p) end -- mismatch in argument types, prime_iter takes {}, number, we are given {}, string

        for p in primes3() do print(p) end -- no error
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let acm =
        type_error_data_ref::<CountMismatch>(&result.errors[0]).expect("expected CountMismatch");
    assert_eq!(CountMismatchContext::Arg, acm.context());
    assert_eq!(2, acm.expected());
    assert_eq!(1, acm.actual());

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
    assert_eq!(fixture.base.get_builtins().numberType, tm.wanted_type);
    assert_eq!(fixture.base.get_builtins().stringType, tm.given_type);
}
