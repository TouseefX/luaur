//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1476:type_infer_provisional_while_loops_fail_to_apply_refinements_1`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_provisional_while_loops_fail_to_apply_refinements_1

#[cfg(test)]
#[test]
fn type_infer_provisional_while_loops_fail_to_apply_refinements_1() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::optional_value_access::OptionalValueAccess;

    crate::DOES_NOT_PASS_OLD_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type walkoptions = {
	recursive: boolean?,
}

function bing(path : string  | walkoptions, opts: walkoptions?)
    return function ()
        while opts and opts.recursive do
        end
    end
end
    "#,
        ),
        None,
    );

    assert!(
        result
            .errors
            .iter()
            .any(|error| type_error_data_ref::<OptionalValueAccess>(error).is_some()),
        "{:?}",
        result.errors
    );
}
