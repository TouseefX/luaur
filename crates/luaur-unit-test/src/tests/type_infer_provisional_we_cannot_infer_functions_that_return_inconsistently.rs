//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1276:type_infer_provisional_we_cannot_infer_functions_that_return_inconsistently`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_provisional_we_cannot_infer_functions_that_return_inconsistently

#[cfg(test)]
#[test]
fn type_infer_provisional_we_cannot_infer_functions_that_return_inconsistently() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function find_first<T>(tbl: {T}, el)
            for i, e in tbl do
                if e == el then
                    return i
                end
            end
            return nil
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "<T>({T}, unknown) -> number",
            to_string_type_id(fixture.require_type_string(&String::from("find_first")))
        );
    } else {
        assert_eq!(
            "<T, b>({T}, b) -> number",
            to_string_type_id(fixture.require_type_string(&String::from("find_first")))
        );
    }
}
