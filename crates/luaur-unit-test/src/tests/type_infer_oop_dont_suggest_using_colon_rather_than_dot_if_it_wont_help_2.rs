//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:50:type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2

#[cfg(test)]
#[test]
fn type_infer_oop_dont_suggest_using_colon_rather_than_dot_if_it_wont_help_2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::count_mismatch::CountMismatch;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local someTable = {}

        local function abs(x: number)
            if x < 0 then
                return -x
            else
                return x
            end
        end

        someTable.Function2 = function(Arg1, Arg2)
            abs(Arg1)
            abs(Arg2)
        end

        someTable.Function2() -- Argument count mismatch
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe { get_type_error::<CountMismatch>(&result.errors[0]).as_ref() };
    assert!(
        err.is_some(),
        "expected CountMismatch, got {:?}",
        result.errors[0]
    );
}
