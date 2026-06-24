//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2877:type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun

#[cfg(test)]
#[test]
fn type_infer_functions_fuzzer_missing_follow_in_ast_stat_fun() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _ = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local _ = function<t0...>()
        end ~= _

        while (_) do
            _,_,_,_,_,_,_,_,_,_._,_ = nil
            function _(...):<t0...>()->()
            end
            function _<t0...>(...):any
                _ ..= ...
            end
            _,_,_,_,_,_,_,_,_,_,_ = nil
        end
    "#,
        ),
        None,
    );
}
